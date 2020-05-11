use rand::Rng;
use std::cmp::Ordering;
use std::io; // Brings in input/out from the Rust standard libray // brings in random number generators

fn main() {
    // println! is a macro and not a method in Rust
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // variables are immutable and mut allows let to mutate
        io::stdin()
            .read_line(&mut guess) // the '&'  is a reference so the guess variable can be used in multiple parts without having to create new memory for it.
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess); // curl bracers {} are a placeholder for guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
