use rand::seq::SliceRandom;
use rand::Rng;
use std::io;
fn generate_unique_number() -> String {
    let mut digits: Vec<char> = ('0'..='9').collect();
    let mut rng = rand::thread_rng();
    digits.shuffle(&mut rng);

    let mut number = String::new();
    for d in &digits {
        if *d != '0' {
            number.push(*d);
            break;
        }
    }

    for d in &digits {
        if !number.contains(*d) {
            number.push(*d);
        }
        if number.len() == 4 {
            break;
        }
    }

    number
}

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("i tesetd on vs code and it worked correctly, used cargo add rand to add the rand crate");
    println!("====================================================");
    println!("Guess the 4-digit number. All digits are unique and the first digit is not 0.");

    let secret = generate_unique_number();

    loop {
        println!("Enter your 4-digit guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = guess.trim();

        if guess.len() != 4 || !guess.chars().all(|c| c.is_ascii_digit()) {
            println!("Please enter a valid 4-digit number.");
            continue;
        }

        if guess.chars().collect::<std::collections::HashSet<_>>().len() != 4 {
            println!("Digits must be unique!");
            continue;
        }

        let mut feedback = Vec::new();

        for (i, g_char) in guess.chars().enumerate() {
            if g_char == secret.chars().nth(i).unwrap() {
                feedback.push(format!("{} - Correct position", g_char));
            } else if secret.contains(g_char) {
                feedback.push(format!("{} - Wrong position", g_char));
            } else {
                feedback.push(format!("{} - Incorrect", g_char));
            }
        }

        println!("Feedback:");
        for msg in &feedback {
            println!("{}", msg);
        }

        if guess == secret {
            println!("Congratulations! You guessed the number: {}", secret);
            break;
        }
    }
}
