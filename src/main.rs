use std::io::{self, Write};
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..101);

    println!("let's guess the number!");
    print!("guess it: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read =(");
    
    println!("you guessed: {}", guess);
    println!("the secret number was {}", secret_number);
}
