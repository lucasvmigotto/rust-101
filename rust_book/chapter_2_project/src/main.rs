use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

    let number: u32 = rand::thread_rng()
        .gen_range(1..=100);

    loop {

        let mut guess: String = String::new();

        println!("What is your guess?");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("Too low..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!(
                    "You got! The number is {} 🥳",
                    number
                );
                break;
            }
        }

    }

}
