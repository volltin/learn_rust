extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Input a number:");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Secret: {}", secret);

    loop {
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("fail to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        match guess.cmp(&secret) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("win!");
                break;
            }
        }
    }
}