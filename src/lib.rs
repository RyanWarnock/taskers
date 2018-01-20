extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate time;

mod task;

pub mod tasklist;

use std::io;
use std::error::Error;
use tasklist::TaskList;

pub fn run(task_list: &mut TaskList) -> Result<(), Box<Error>> {
    task_list.task_list_location = TaskList::get_task_file_location()?;
    task_list.load_from_file()?;
    task_list.print_tasks();
    loop {
        match select_mode()? {
            Mode::Add       => task_list.add_task_prompt()?,
            Mode::Remove    => task_list.remove_task()?,
            Mode::Complete  => task_list.complete_task()?,
            Mode::Quit      => break,
            Mode::Invalid   => println!("Invalid mode selection"),
        };
        task_list.save_to_file()?;
        task_list.print_tasks();
    }
    Ok(())
}

pub fn select_mode() -> Result<Mode, Box<Error>> {
    println!("Select mode: (a)dd (r)emove (c)omplete (q)uit");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(match input.trim() {
           "a" => Mode::Add,
           "r" => Mode::Remove,
           "c" => Mode::Complete,
           "q" => Mode::Quit,
           _ => Mode::Invalid,
       })
}

pub enum Mode {
    Add,
    Remove,
    Complete,
    Quit,
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_test() {
        assert_eq!(1, 1);
    }
}
