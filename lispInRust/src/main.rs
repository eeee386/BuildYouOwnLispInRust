use std::io::{self, Write};
use std::env;

fn main() {
    let mut input: String = String::new();
    let args: Vec<String> = env::args().collect();
    println!("Lispy Version 0.0.0.0.1");
    println!("Press CTRL+C to exit");

    // In a never ending loop
    loop {
        // Output our prompt
        io::stdout().write_all(b"Lispy>").unwrap();
        
        // Read a line of user input of maximum size 2048
        io::stdin().read_line(&mut input).unwrap();
        
        // Echo input back to user input
        println!("No, you are a {}", input);
    }
}
