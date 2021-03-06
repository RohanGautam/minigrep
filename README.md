## Learnings:

- Using primitive values when a complex type would be more appropriate is an anti-pattern known as primitive obsession. It's better to abstract away things into structs/object-like things sometimes.
- lifetime params specify which argument lifetime is connected to the lifetime of the return value.
- if you dont print to `stderr`, running `cargo run > temp.txt` will redirect the error message to the file, as `println!` prints to `stdout`. We want to see the error instead, by printing it to `stderr` instead. so, we use `eprintln!` instead!

#

## Seperation of code

- For example, a config variable containing all config stuff would be more appropriate than a bunch of vars scattered around.
  The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. The process has the following steps:

  - Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
  - As long as your command line parsing logic is small, it can remain in main.rs.
  - When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

- The responsibilities that remain in the main function after this process should be limited to the following:

  - Calling the command line parsing logic with the argument values
  - Setting up any other configuration
  - Calling a run function in lib.rs
  - Handling the error if run returns an error

This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand.

#

> It’s better to have a working program that’s a bit inefficient than to try to hyperoptimize code on your first pass.
