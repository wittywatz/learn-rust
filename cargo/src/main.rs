extern crate anyhow;
extern crate rand;

#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use rand::Rng;
use std::io::stdin;

fn main() -> Result<()> {
    // Generate a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        stdin().read_line(&mut guess)?;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    Ok(())
}
