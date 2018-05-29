use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();;
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    let mut num = input.trim().parse().expect("Not a number!");
    let mut result = vec![1, 1];

    match num {
        1 => println!("{}", 1),
        2 => println!("{:?}", result),
        _ => fibonacci(result, num),
    };

    fn fibonacci(mut vec: Vec<u32>, num: u32) -> () {
        for x in 0..num - 4{
            let mut item = vec[x];
            let mut item2 = vec[x+1];
            vec.push(item + item2);
        }
        println!("{:?}", vec);
    }
}
