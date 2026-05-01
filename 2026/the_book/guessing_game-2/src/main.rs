use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guessing Game!!!!");

    // let secret_number: u32 = 197;
    let secret_number = rand::thread_rng().gen_range(0..=100);

    println!("Guess the secret number! - It is {}", secret_number);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("What did you enter? I failed to read it!");

        let guessed_num: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed {guessed_num}!");

        match guessed_num.cmp(&secret_number) {
            Ordering::Less => println!("It's bigger!"),
            Ordering::Greater => println!("It's smaller!"),
            Ordering::Equal => {
                println!("Bingo!!!!");
                break;
            }
        }
    }
}
