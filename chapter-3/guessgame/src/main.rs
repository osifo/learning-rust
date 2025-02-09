use rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;

fn guessing_game() -> () {
    let rand_number: i32 = rand::rng().random_range(1..=100);
    let mut game_over = false;

    println!("{}", rand_number);

    println!("enter a number");

    while !game_over {
        let mut user_guess = String::new();
        stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        let user_guess: i32 = user_guess.trim().parse().expect("Please enter a number.");
    
        let result_message = match user_guess.cmp(&rand_number) {
            Ordering::Less => "Guess a higher number",
            Ordering::Greater => "Guess a lower number",
            Ordering::Equal => {
                game_over = true;
                "Correct!"
            }
        };
        
        println!("{}", result_message)
    }
}

fn main() {
    guessing_game();
}

