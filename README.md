# factorial-rs
A rewrite of [FactorialMultithread](https://github.com/WCBROW01/FactorialMultithread) in Rust using rug, a high-level API for using GMP in Rust.

This program calculates factorials by segmenting them across multiple CPU threads. It does this very efficiently, with minimal time being spent by the main thread waiting for other threads to finish since it will start piecing the final number together immediately after a thread is done with its operation. This code is thread-safe, unlike the Java code, since it uses channels to acquire the result from each thread rather than spinning the threads and accessing their memory directly. It is also multiple times faster than the Java code and uses much, much less memory.

### Building the program
To build factorial-rs, there are a few prerequisites.

Your system needs the rust compiler and cargo package manager installed using rustup, as well as a C compiler, diffutils, m4, and make.

If you're running Linux, you'll want to install these from your package manager of choice.

Example: `sudo apt-get install diffutils gcc m4 make`

On Windows, you can get these using mingw-w64 and its package manager.

If you're on macOS, you'll find everything you need other than rust installed with the xcode command-line developer tools.

Once you have all of the build tools installed, run `cargo build --release` and you'll have an executable!

### Usage information
To get usage information, just run the program with the `--help` flag!

`-i` or `--interactive` will start the program in interactive mode

`-n` or `--number` will immediately generate the desired factorial, with the next parameter being the number you want.

`-t` or `--threads` will run the program with the desired number of threads.

`-p` or `--print` will print the generated factorial to the console. (WARNING: COULD BE VERY SLOW)