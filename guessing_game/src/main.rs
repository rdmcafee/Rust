use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    
    println!("Please input your guess.");

    let mut guess = String::new(); //mut stands for mutable 

    io::stdin()
        .read_line(&mut guess) //&mut indicates a reference to the mutable variable guess
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
