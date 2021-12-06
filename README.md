# factorial-rs
A rewrite of [FactorialMultithread](https://github.com/WCBROW01/FactorialMultithread) in Rust using num-bigint. Doesn't yet achieve feature parity with the old project, but this is coming soon!

This program calculates factorials by segmenting them across multiple CPU threads. It does this very efficiently, with minimal time being spent by the main thread waiting for other threads to finish since it will start piecing the final number together immediately after a thread is done with its operation. This code is thread-safe, unlike the Java code, since it uses channels to acquire the result from each thread rather than spinning the threads and accessing their memory directly. It is also multiple times faster than the Java code and uses much, much less memory.

### Building the program
To build factorial-rs, all you need to do is have `cargo` installed and run`cargo build --release`

### Usage information
To get usage information, just run the program with the `--help` flag!

`-i` or `--interactive` will start the program in interactive mode (NOT IMPLEMENTED YET)

`-n` or `--number` will immediately generate the desired factorial, with the next parameter being the number you want.

`-t` or `--threads` will run the program with the desired number of threads.

`-p` or `--print` will print the generated factorial to the console. (WARNING: COULD BE VERY SLOW)