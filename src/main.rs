extern crate rand;
pub mod facil;

use std::cmp::Ordering;
use rand::Rng;
use facil::*;

fn main() {
    // overture
    banner("GUESS THE NUMBER");

    // game loop
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        let mut counter = 1;

        // guessing loop
        loop {
            let mut guess: i32;

            // input loop
            loop {
                print!("Guess {counter} - Type your guess (1 - 100): ");
                guess = input_i32();
                if guess < 1 {
                    println!("Invalid guess! Type again!");
                } else { break }
            }

            // decision and output
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Very low!");
                    counter += 1;
                },
                Ordering::Greater => {
                    println!("Very high!");
                    counter += 1;
                },
                Ordering::Equal => {
                    println!("You got in {counter} guesses!");
                    break;
                },
            }
        }

        // play again
        if !again("Play again?[yes/no]: ", "yes", "no") { 
            break;
        }
    }

    // the end
    banner("Thanks for play GUESS THE NUMBER!")
}
