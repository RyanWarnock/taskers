extern crate taskers;

use taskers::TaskList;
use std::io::prelude::*;
use std::process;

fn main() {
    println!("Taskers: The tacky task tracker for tiresome tasks");

    let mut task_list = TaskList::new();
    let mut stderr = std::io::stderr();

    if let Err(err) = task_list.load_from_file() {
        writeln!(&mut stderr, "File handling error: {}", err)
            .expect("Couldn't write to stderr");
        process::exit(1);
    } 

    if let Err(err) = taskers::run(&mut task_list) {
        writeln!(&mut stderr, "Application error: {}", err)
            .expect("Couldn't write to stderr");
        process::exit(1);
    }
}
