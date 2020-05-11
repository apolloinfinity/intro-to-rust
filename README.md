# Intro to Rust
My notes on learning the ins and outs of the Rust programming language. 

The first example 'hello_world/' was created without using cargo. 
I simply created the file called main.rs and the typed in the terminal:
``` bash
$ # Within the same directory as main.rs
$ rustc main.rs 
```
Using 'rustc' in the terminal runs the .rs file. In Rust, compiling and running are two seperate things. 


To create a new project using the Cargo command type in the terminal: 
```bash
$ cargo new hello_cargo
$ cd hello_cargo
```

## Building and running a Cargo project
```bash 
$ cargo build # This command creates an executable in target directory
```
cargo build creats a Cargo.lock file that keeps track of the versions and dependencies of the project. Cargo build only builds from the source file. 

To build and run a file, the command:
```bash 
$ cargo run
```

To check your code without creating an executable:
```bash
$ cargo check 
```


Tutorial for the Guessing Game can be found [Here](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)