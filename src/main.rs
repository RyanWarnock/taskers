extern crate taskers;

use taskers::tasklist::TaskList;
use std::io::prelude::*;
use std::process;

// Currently handles all errors and just calls process::exit(1)
// whenever an error occurs.
fn main() {
    println!("Taskers: The tacky task tracker for tiresome tasks");

    let mut task_list = TaskList::new();
    let mut stderr = std::io::stderr();

    if let Err(err) = taskers::run(&mut task_list) {
        writeln!(&mut stderr, "Application error: {}", err)
            .expect("Couldn't write to stderr");
        process::exit(1);
    }
}
