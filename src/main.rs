mod fact_thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use num_bigint::BigUint;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() == 1 || args[1] == "-i" || args[1] == "--interactive" {
		println!("1 arg, doing interactive");
	} else if args[1] == "h" || args[1] == "--help" {
		println!("{}", help());
	} else {
		let mut number = 0;
		// the default, 0, will be expand the the number of CPU threads
		let mut thread_count = 0;
		let mut printing = false;

		if args.len() < 3 {
			println!("Invalid number of arguments.");
			std::process::exit(1);
		}

		// Iterate through arguments
		for argument in 1..args.len() {
			if args[argument] == "-n" || args[argument] == "--number" {
				match args[argument + 1].parse::<usize>() {
					Ok(n) => number = n,
					Err(_) => invalid_args(&args[argument])
				}
			} else if args[argument] == "-t" || args[argument] == "--threads" {
				match args[argument + 1].parse::<usize>() {
					Ok(n) => thread_count = n,
					Err(_) => invalid_args(&args[argument])
				}
			} else if args[argument] == "-p" || args[argument] == "--print" {
				printing = true;
			}
		}

		let result = gen_factorial(number, thread_count);
		println!("Successfully generated {}!", number);
		if printing {
			println!("{}", result);
		}
	}
}

// Exits gracefully if an invalid argument is encountered
fn invalid_args(arg_type: &str) {
	println!("Invalid argument for \"{}\". Check usage with \"--help\".", arg_type);
	std::process::exit(1);
}

// Returns usage information for the program as a &str
fn help() -> &'static str {
	return "Usage: factorial-rs [OPTIONS]
This program generates a factorial with parallel processing!

Options:
-i, --interactive	Start in interactive mode. Default if no arguments are passed. (NOT IMPLEMENTED YET)
-n, --number NUMBER	Input number to calculate the factorial of.
-t, --threads THREADS	Number of threads to calculate the factorial with. (Automatically determined if not passed)
-p, --print		Print the generated factorial to the screen.";
}

/* Initializes the threads to generate the factorial
 * and collects the results from each thread */
fn gen_factorial(number: usize, mut thread_count: usize) -> BigUint {
	let mut result = BigUint::new(vec![1]);

	/* If the input is less than 1 (either 0 or a negative number),
	 * grab the number of available processors and use that. */
	if thread_count < 1 {
		thread_count = num_cpus::get();
	}

	/* If we have a greater quantity of threads than numbers to multiply,
	 * fall back to 1 thread. */
	if number < thread_count {
		thread_count = 1;
	}

	// Create an vector the size of the thread count.
	let (tx, rx): (Sender<BigUint>, Receiver<BigUint>) = mpsc::channel();
	let mut threads = Vec::with_capacity(thread_count);

	// Start each thread.
	let mut section_start = 0;
	for _ in 0..thread_count {
		let thread_tx = tx.clone();

		let current_thread = thread::spawn(move || {
			let start = section_start + 1;
			section_start += number / thread_count;
			let end = section_start;
			let thread_result = fact_thread::run(start, end);
			thread_tx.send(thread_result).unwrap();
		});

		threads.push(current_thread);
	}

	let mut threads_remaining = thread_count;
	while threads_remaining > 0 {
		match rx.try_recv() {
			Ok(thread_result) => {
				result *= thread_result;
				threads_remaining -= 1;
			},
			Err(_) => {}
		}
	}

	return result;
}
