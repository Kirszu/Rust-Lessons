use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();;
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    let num = input.trim().parse().expect("Not a number!");
    let result = vec![1, 1];

    match num {
        1 => println!("{}", 1),
        2 => println!("{:?}", result),
        _ => fibonacci(result, num),
    };

    fn fibonacci(mut vec: Vec<u32>, num: u32) -> () {
        let mut x = 1;
        let mut y = 1;
        loop {
            if x + y >= num {
                break;
            }
            vec.push(x + y);
            let temp = x;
            x = y;
            y = temp + y;
        }
        println!("{:?}", vec);
    }
}
