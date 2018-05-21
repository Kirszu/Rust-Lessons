use std::io;
fn main() {
    // PLN = 0.23EUR
    // EUR = 4.29PLN
    println!("Choose 1 to convert PLN to EUR");
    println!("Choose 2 to convert EUR to PLN");
    let mut input = String::new();;
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    let eur = 4.29;
    let pln = 0.23;
    let num: u32 = input.trim().parse().expect("Not a number!");

    println!("Choose amount you want to convert");
    let mut input2 = String::new();;
    io::stdin().read_line(&mut input2)
    .expect("Failed to read line");
    let amount: f32 = input2.trim().parse().expect("Not a number!");

    match num {
        1 => convert(amount, pln),
        2 => convert(amount, eur),
        _ => println!("Choose 1 or 2!"),
    }
}

fn convert(amount: f32, multiplier: f32) {
    println!("Result {}", amount * multiplier);
}
