use std::{io::{self, Write}, cmp::Ordering};
use rand::Rng;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(0..101);

    println!("let's guess the number!");
    loop {   
        print!("guess it: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read =(");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a positive integer");
                continue;
            }
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("ᵗᵒᵒ ᵐᵘᶜʰ"),
            Ordering::Less => println!("𝗠  𝗢  𝗥  𝗘 !"),
            Ordering::Equal => {
                println!("you won, congratulation :D");
                break;
            },
        }
    }
    println!("the secret number was {}", secret_number);
}
