use std::io;

fn main() {
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    name = name.trim().to_string();

    println!("Enter your guess number: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("🥳 {name}, you guessed: {guess}");
}
