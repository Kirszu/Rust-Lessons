extern crate rand;

use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(1, 4);
    match num {
        1 => println!("First option!"),
        2 => println!("Second option!"),
        3 => println!("Third option!"),
        _ => println!("???"),
    };
}
