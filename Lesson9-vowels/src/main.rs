use std::io;

fn main() {
    println!("Enter a string:");
    let mut input = String::new();;
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");

    let mut cons = 0;
    let mut vowels = 0;
    for x in input.chars() {
        if x.is_alphabetic() {
            match x {
                'a' | 'e' | 'y' | 'u' | 'i' | 'o' | 'A' | 'E' | 'Y' | 'U' | 'I' |'O'  => vowels += 1,
                _ => cons += 1,
            };
        }
    }
    println!("The string contains {0} vowels and {1} consonants.", vowels, cons);
}
