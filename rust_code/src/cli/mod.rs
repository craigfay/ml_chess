
use std::io::{stdin,stdout,Write};


pub fn get_input(prompt: String) -> String {
    // Write a prompt to the terminal
    print!("{}", prompt);

    // Read a line of input from the terminal
    let mut input = String::new();
    stdout().flush();
    stdin().read_line(&mut input).expect("Invalid Input!");


    // Pop off newline characters
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    
    input
}

