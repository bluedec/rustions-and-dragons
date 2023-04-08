use std::thread;
use std::time::Duration; 
use magic::Class;
use std::io;

pub fn take_inp() -> String {
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input.");

    input2.trim().to_lowercase().to_string()
}

fn main() {
    println!("<<<0/ O \\0>>>Welcome to Rustions and Dragons");        
    println!("1) Start");
    let inp = take_inp();
    if inp ==
}


