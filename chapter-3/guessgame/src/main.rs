use rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;

fn guessing_game() -> () {
    let rand_number = rand::rng().random_range(1..=100);
        
    println!("A number has been generated");
    println!("guess the number");

    
    loop {
        let mut user_guess = String::new();
        stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("guess the number");
                continue;
            }, //moves to the next iteration of the loop
        };
    
        let result_message = match user_guess.cmp(&rand_number) {
            Ordering::Less => "Guess a higher number",
            Ordering::Greater => "Guess a lower number",
            Ordering::Equal => "Correct!"
        };
        
        println!("{}", result_message);

        if result_message == "Correct!" { break; };
    }
}

fn main() {
    guessing_game();
}

