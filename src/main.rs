use std::io;

fn main() {
    println!("Taskers: The tacky task tracker for tiresome tasks");
    let mut command = String::new();
    io::stdin().read_line(&mut command)
        .expect("Could not read line");
}
