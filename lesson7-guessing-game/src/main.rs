extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number:");
    let mut input = String::new();;
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");

    let guess: u32 = input.trim().parse().expect("Not a number!");

    let num = rand::thread_rng().gen_range(1, 5);
    println!("Num is {}", num);
    while num != guess{
        if num < guess {
            println!("The number is lower than your guess! Try again");
            let mut input = String::new();;
            io::stdin().read_line(&mut input)
            .expect("Failed to read line");
            let guess: u32 = input.trim().parse().expect("Not a number!");
            println!("Guess is: {}", guess);
            println!("Num is: {}", num);
        } else if num > guess {
            println!("The number is higher than your guess! Try again");
            let mut input = String::new();;
            io::stdin().read_line(&mut input)
            .expect("Failed to read line");
            let guess: u32 = input.trim().parse().expect("Not a number!");
            println!("Guess is: {}", guess);
            println!("Num is: {}", num);
            } else {
            println!("You found a number!");
            }
        }
}
