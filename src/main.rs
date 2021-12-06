mod fact_thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::io::{self, Write};
use num_bigint::BigUint;

fn main() {
	let mut number = 0;
	// the default, 0, will be expand the the number of CPU threads
	let mut thread_count = 0;
	let mut printing = false;
	let args: Vec<String> = std::env::args().collect();
	if args.len() == 1 || args[1] == "-i" || args[1] == "--interactive" {
		let params_tuple = interactive();
		number = params_tuple.0;
		thread_count = params_tuple.1;
		printing = params_tuple.2;
	} else if args[1] == "h" || args[1] == "--help" {
		println!("{}", help());
	} else {
		if args.len() < 3 {
			println!("Invalid number of arguments.");
			std::process::exit(1);
		}

		// Iterate through arguments
		for argument in 1..args.len() {
			match &*args[argument] {
				"-n" | "--number" => {
					match args[argument + 1].parse::<usize>() {
						Ok(n) => number = n,
						Err(_) => invalid_args(&args[argument])
					}
				}
				"-t" | "--threads" => {
					match args[argument + 1].parse::<usize>() {
						Ok(n) => thread_count = n,
						Err(_) => invalid_args(&args[argument])
					}
				}
				"-p" | "--print" => printing = true,
				_ => {} // Do nothing
			}
		}
	}

	let result = gen_factorial(number, thread_count);
	println!("Successfully generated {}!", number);
	if printing {
		println!("{}", result);
	}
}

// Exits gracefully if an invalid argument is encountered
fn invalid_args(arg_type: &str) -> ! {
	println!("Invalid argument for \"{}\". Check usage with \"--help\".", arg_type);
	std::process::exit(1);
}

// Returns usage information for the program as a &str
fn help() -> &'static str {
"Usage: factorial-rs [OPTIONS]
This program generates a factorial with parallel processing!

Options:
-i, --interactive	Start in interactive mode. Default if no arguments are passed.
-n, --number NUMBER	Input number to calculate the factorial of.
-t, --threads THREADS	Number of threads to calculate the factorial with. (Automatically determined if not passed)
-p, --print		Print the generated factorial to the screen."
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
	for thread_num in 0..thread_count {
		let thread_tx = tx.clone();

		let current_thread = thread::spawn(move || {
			let start = thread_num * number / thread_count + 1;
			let end = (thread_num + 1) * number / thread_count;
			let thread_result = fact_thread::run(start, end);
			thread_tx.send(thread_result).unwrap();
		});

		threads.push(current_thread);
	}

	let mut threads_remaining = thread_count;
	while threads_remaining > 0 {
		if let Ok(thread_result) = rx.try_recv() {
			result *= thread_result;
			threads_remaining -= 1;
		}
	}

	result
}

fn interactive() -> (usize, usize, bool) {
	let mut number = 0;
	let mut thread_count = 0;
	let mut print = false;
	let mut input = String::new();

	print!("How many threads do you want to use? (0 to automatically allocate): ");
	input.clear();
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).unwrap();
	input = input.trim().to_owned();
	match input.parse::<usize>() {
		Ok(t) => thread_count = t,
		Err(_) => println!("Invalid number!")
	}

	print!("Enter number to complete factorial: ");
	input.clear();
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).unwrap();
	input = input.trim().to_owned();
	match input.parse::<usize>() {
		Ok(n) => number = n,
		Err(_) => println!("Invalid number!")
	}

	print!("Would you like to print the result? ");
	input.clear();
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).unwrap();
	input = input.to_lowercase().trim().to_owned();
	match &*input {
		"yes" | "y" => print = true,
		"no" | "n" => print = false,
		_ => println!("Invalid input")
	}

	(number, thread_count, print)
}