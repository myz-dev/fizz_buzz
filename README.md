# Summary
This workspace explores an implementation of the famous `FizzBuzz` game that
focuses on flexibility and extensibility.

# Build
Run `cargo build` to build the CLI library.

# Run
Run the binary with `cargo run`. To pass CLI arguments to the executable, you can 
run it like this: `cargo run -- --help`.

# Implementation details
The trait `TokenCondition` describes the interface a type has to implement, in order to 
participate in the `FizzBuzz` game.
How this trait works and how it is used to power the game, you can see the tests 
[here](./lib/fizz_buzz/src/test.rs)
