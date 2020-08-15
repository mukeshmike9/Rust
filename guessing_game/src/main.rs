use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
/**
 * Guessing Game
 * User has to guess numbers and try to figure out the secret number
 * */
fn main() {
    println!("Guess The Number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("Secret Number is: {}", secret_number);

    loop {
        println!("\nInput your guess");
        print!("User Input => ");
        io::stdout().flush().expect("StdOut failed...!!!"); //This will not wait for OS to flust the STD out buffer and force it to print
        let mut guess = String::new();

        /*
         *  Get an Input from User
         */
        let std_in_object = io::stdin(); // Return std::io::Stdin object
        let result = std_in_object.read_line(&mut guess); //Returns std::io::Result object
        result.expect("Failed to read Line"); // Handle error if read fails

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("Correct: You Win!");
                break;
            }
        }
    }
}
