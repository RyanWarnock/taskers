extern crate taskers;

use std::io;

use taskers::Command;

fn main() {
    println!("Taskers: The tacky task tracker for tiresome tasks");
    let mut command = String::new();
    let quit = false;
    while !quit {
        io::stdin().read_line(&mut command)
            .expect("Could not read line");
        if let Err(e) = Command::new(&command) {
            println!("{}", e);    
        } else {
            println!("Command valid");    
        }
    }
}
