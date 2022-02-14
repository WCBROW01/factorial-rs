use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, mpsc};
use std::thread;
use std::io::{self, Write};
use rug::{Integer, Assign};

fn main() {
	let mut number = 0;
	let mut printing = false;
	let args: Vec<String> = std::env::args().collect();
	if args.len() == 1 || args[1] == "-i" || args[1] == "--interactive" {
		let params_tuple = interactive();
		number = params_tuple.0;
		printing = params_tuple.1;
	} else if args[1] == "h" || args[1] == "--help" {
		println!("{}", help());
	} else if args.len() < 3 {
		eprintln!("Invalid number of arguments.");
		std::process::exit(1);
	}

	// Iterate through arguments
	for argument in 1..args.len() {
		match &*args[argument] {
			"-n" | "--number" => {
				number = match args[argument + 1].parse::<usize>() {
					Ok(n) => n,
					Err(_) => invalid_args(&args[argument])
				};
			}
			"-p" | "--print" => printing = true,
			_ => {} // Do nothing
		}
	}

	let result = gen_factorial(number);
	println!("Successfully generated {}!", number);
	if printing {
		println!("{}", result);
	}
}

/// Exits gracefully if an invalid argument is encountered
fn invalid_args(arg_type: &str) -> ! {
	eprintln!("Invalid argument for \"{}\". Check usage with \"--help\".", arg_type);
	std::process::exit(1);
}

/// Returns usage information for the program as a &str
fn help() -> &'static str {
"Usage: factorial-rs [OPTIONS]
This program generates a factorial with parallel processing!

Options:
-i, --interactive	Start in interactive mode. Default if no arguments are passed.
-n, --number NUMBER	Input number to calculate the factorial of.
-p, --print		Print the generated factorial to the screen."
}

fn usqrt(n: usize) -> usize {
	let mut x = n / 2;
	let mut x_last = x;
	let mut x_last2;

	// cursed loop cursed loop cursed loop (this acts like a do-while loop)
	while {
		x_last2 = x_last;
		x_last = x;
		x = (x + n / x) / 2;
		x != x_last && x != x_last2
	} {}

	x
}

/// Precomputes a list of how many threads each thread should spawn (in a tree)
fn gen_thread_list(number: usize) -> Vec<usize> {
	fn generate(n: usize, list: &mut Vec<usize>) {
		if n >= 16 {
			generate(usqrt(usqrt(n)), list);
		}

		list.push(n);
	}

	let mut list = Vec::new();
	generate(usqrt(usqrt(number)), &mut list);
	list
}

fn gen_factorial(number: usize) -> Integer {
	if number > 16 {
		let thread_list = Arc::new(gen_thread_list(number));
		gen_threads(0, number, thread_list, 0)
	} else {
		factorial(1, number)
	}
}

/// Initializes the threads used to generate the factorial and collects the results from each thread
fn gen_threads(start: usize, end: usize, thread_list: Arc<Vec<usize>>, level: usize) -> Integer {
	let length = end - start;
	let thread_count = thread_list[level];
	let mut result = Integer::new();
	result.assign(1);

	// Create a channel for sending/recieving the thread results
	let (tx, rx): (Sender<Integer>, Receiver<Integer>) = mpsc::channel();

	// Start each thread.
	for thread_num in 0..thread_count {
		let thread_list = thread_list.clone();
		let thread_tx = tx.clone();

		thread::spawn(move || {
			let thread_start = thread_num * length / thread_count + start + 1;
			let thread_end = (thread_num + 1) * length / thread_count + start;
			let thread_result = if level < thread_list.len() - 1 {
				gen_threads(thread_start, thread_end, thread_list, level + 1)
			} else {
				factorial(thread_start, thread_end)
			};

			match thread_tx.send(thread_result) {
				Ok(_) => {},
				Err(_) => panic!("Failed to send result to parent thread.")
			}
		});
	}

	for _ in 0..thread_count {
		match rx.recv() {
			Ok(n) => result *= n,
			Err(_) => panic!("Failed to retrieve result from a child thread.")
		}
	}

	result
}

/// Entrypoint for each of the top-level factorial threads
fn factorial(start: usize, end: usize) -> Integer {
	let mut section = Integer::new();
	section.assign(1);
	for count in start..end+1 {
		section *= count as u64;
	}

	section
}

/// Interactive REPL for initializing the program
fn interactive() -> (usize, bool) {
	let mut input = String::new();

	print!("Enter number to complete factorial: ");
	input.clear();
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).unwrap();
	input = input.trim().to_owned();
	let number = match input.parse::<usize>() {
		Ok(n) => n,
		Err(_) => {
			eprintln!("Invalid number!");
			std::process::exit(1);
		}
	};

	print!("Would you like to print the result? ");
	input.clear();
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).unwrap();
	input = input.to_lowercase().trim().to_owned();
	let print = match &*input {
		"yes" | "y" => true,
		"no" | "n" => false,
		_ => {
			eprintln!("Invalid input");
			std::process::exit(1);
		}
	};

	(number, print)
}
