use std::io;
use std::io::prelude::*;

/*
 * This function allows build a line of characters with a give character. 
 * ch - the character
 * t - number of repeats
 */
pub fn repeat(ch: char, t: usize) -> String {
    let mut repeated = String::new();

    for _ in 0..t {
        repeated.push(ch);
    }

    repeated
}


/*
 * This function builds a banner.
 */
pub fn banner(message: &str) {
    const LENGTH: usize = 50;

    let line = repeat('-', LENGTH);
    let spaces = repeat(' ', (LENGTH - message.len()) / 2);

    println!("\n{line}\n{spaces}{message}{spaces}\n{line}\n");    
}


/*
 * This function allows input of a 32-bits integer with error handling.
 * Wrong answers returns 0. The Guessing Game has inputs 1 - 100.
 */
pub fn input_i32() -> i32 {
    let mut str = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut str).unwrap();

    let value: i32 = match str.trim().parse::<i32>() {
        Ok(value) => value, // valor correto
        Err(_) => 0,        // valor com qq tipo de erro
    };

    value
}


/*
 * This function allow input of a string.
 */
pub fn input() -> String {
    let mut s = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).unwrap();

    // retira <ENTER> da string
    if let Some('\n') = s.chars().next_back() { s.pop(); }
    if let Some('\r') = s.chars().next_back() { s.pop(); }

    s
}


/*
 * This function shows a message in the screen and catch  one of two given options:
 * Example:
 * mensage: "Play again?[yes/no]"
 * option 1: "yes"
 * option 2: "no"
 * The function expects typing one of the options and do not accept other. The first
 * returns true and the second returns false. . 
 */
pub fn again(message: &str, op1: &str, op2: &str) -> bool {
    let mut choice: String = String::from("_");

    while choice != op1 && choice != op2 {
        print!("{}", message);
        choice = input();        
    }

    choice == op1
}
