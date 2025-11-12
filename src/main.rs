use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secert_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secert_number}");

    loop { // infinite loop until we break out of it
        println!("Please input your guess");
        let mut guess = String::new(); // In rust the variables are immutable by default, 
        // so we use mut keyword to make it mutable

        io::stdin().read_line(&mut guess).expect("Failed to read"); // read_line() takes a mutable reference to the string
        let guess: u32 = match guess.trim().parse() { // trim() removes whitespace and parse() converts string to number
            Ok(num) => num,
            Err(_) => { // _ is a catch-all value, we don't care about the specific error
                println!("Please enter a valid number!");
                continue
            } 
        }; 
        // Rust allows shadowing, so we can reuse the variable name guess rather than creating a new variable i.e. let guess2
        println!("You guessed {guess}");

        match guess.cmp(&secert_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        //immutable variables, constants are values that are bound to a name and are not allowed to change, 
        // but there are a few differences between constants and variables.
    }
}


