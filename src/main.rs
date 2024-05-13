use std::io;
use rand::Rng;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {secret_number}");
    
    println!("__________________________________________________________________________________");

    println!("Enter your name: ");
    let mut name: String = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    fn guess(mut name: String, secret_number: i32, trials: i32) {
    
        name = name.trim().to_string();
    
        println!("Enter your guess number: ");
        let mut guess_number = String::new();

        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");
    
        println!("Hey {name}, you guessed: {guess_number}");

        if guess_number.trim().to_string() == secret_number.to_string() {
            println!("🥳 {name}, you win!");
            return;
        } else {
            if trials < 5 {
                println!("🙃 Na Na.. try again! ({trials})");
                guess(name, secret_number, trials + 1);
            } else {
                println!("😭 {name}, you lose! all trials are used, the secret number is {secret_number}, you can try again later!");
            }
        }
    }

    guess(name, secret_number, 1);

}
