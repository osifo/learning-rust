extern crate rand;

use std::cmp::Ordering;
use std::io;
use rand::Rng;


fn main() {
    let secret_number =  rand::thread_rng().gen_range(1, 101);

    println!("Guess the number");

    loop {
        println!("Input your guess");

        let mut user_guess = String::new();

        io::stdin().read_line(&mut user_guess)
            .expect("Failed to read line");
        
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_err) => continue,
        };

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Guessed number is too small."),
            Ordering::Greater => println!("Guessed number is too large."),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        };
    }
}                                                                                            