use std::io;

pub fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    
    user_input.trim().to_string()
}

pub fn parse_user_input(input: String) -> u32 {
    input.parse::<u32>().expect("Invalid input")
}