
use std::io::{stdin,stdout,Write};


pub fn get_input(prompt: &str) -> String {
    // Write a prompt to the terminal
    print!("{}", prompt);

    // Read a line of input from the terminal
    let mut input = String::new();
    stdout().flush().expect("Could not flush stdout");
    stdin().read_line(&mut input).expect("Invalid Input!");

    // Pop off newline characters
    let last_char = input.chars().next_back();
    if last_char == Some('\n') || last_char == Some('\r') {
        input.pop();
    }
    
    input
}

