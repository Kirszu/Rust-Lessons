extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number:");

    let mut guess: u32 = get_input();
    let num = rand::thread_rng().gen_range(1, 101);

    while num != guess{
        if num < guess {
            println!("The number is lower than your guess! Try again");
            guess = get_input();
        } else if num > guess {
            println!("The number is higher than your guess! Try again");
            guess = get_input();
            }
        }
    println!("You guessed the number! It was: {}", num);
}

fn get_input() -> u32 {
    let mut input = String::new();;
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    let x = input.trim().parse().expect("Not a number!");
    return x;
}
