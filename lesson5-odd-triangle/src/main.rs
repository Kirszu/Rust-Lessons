use std::io;
fn main() {
    println!("Enter a number:");
    let mut input = String::new();;
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");

    let mut result = String::from("*");

    let num: u32 = input.trim().parse().expect("Not a number!");
    for x in 0..num {
        if x % 2 == 0 {
            println!("{}", result);
        }
        result.push('*');
    }
}
