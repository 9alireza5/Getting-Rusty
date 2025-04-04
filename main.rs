use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Number Game!");
    println!("Please guess the number. it's between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is smaller than exact number."),
            Ordering::Greater => println!("Your guess is bigger than exact number."),
            Ordering::Equal => {
                println!("Congrats! You guessed the number.");
                break;
            }
        }
    }
}
