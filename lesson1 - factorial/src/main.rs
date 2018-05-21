use std::io;
fn main() {
    println!("Enter a number:");
    let mut input = String::new();;
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");

    let num: u32 = input.trim().parse().expect("Not a number!");
    println!("Result:  {}", factorial(num));
}

fn factorial(num: u32) -> u32 {
    if num <= 1 {
    return 1;
  }
  num * factorial(num - 1)
}
