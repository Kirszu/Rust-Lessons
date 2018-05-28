use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();;
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    let mut num = input.trim().parse().expect("Not a number!");

    fibonacci(num);

    fn fibonacci(num: u32) -> u32 {
        println!("{}", num);
        if num == 0 || num == 1  {
            return num;
        } else {
            return fibonacci(num - 1) + fibonacci(num - 2);
        }
    }    
}
