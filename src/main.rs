use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess, the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is:{}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing, remove 5\n, parse error
        // let guess: u32 = guess.trim().parse().expect("Please type a number");

        // match method
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(" is Too small!"),
            Ordering::Greater => println!(" is Too big!"),
            Ordering::Equal => {
                println!(" You win!");
                break;
            }
        }
    }
}
