use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1, 101);

    loop {
        println!("Guess a number between 1 and 100");
        println!("Please input your guess: ");

        // 1. `::` indicates a static method/an associator
        // 2. `String::new()` returns new string instance
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
