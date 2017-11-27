extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::io;
use std::fmt;
use std::fs::OpenOptions;
use std::error::Error;

pub fn run(task_list: &mut TaskList) -> Result<(), Box<Error>> {
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

impl TaskList {
    pub fn new() -> TaskList {
        let task_list: Vec<Task> = Vec::new();
        TaskList { task_list }
    }

    pub fn load_from_file(&mut self) -> Result<(), Box<Error>> {
        let f = OpenOptions::new().read(true)
            .write(true)
            .create(true)
            .open("task.list")?;
        let mut rdr = csv::Reader::from_reader(f);
        for result in rdr.deserialize() {
            let task: Task = result?;
            self.task_list.push(task);
        }
        Ok(())
    }

    pub fn save_to_file(&self) -> Result<(), Box<Error>> {
        let mut wtr = csv::Writer::from_path("task.list")?;
        wtr.flush()?;
        for task in &self.task_list {
            wtr.serialize(&task)?;
            wtr.flush()?;
        }
        Ok(())
    }

    pub fn print_tasks(&self) {
        println!("---------------------------------");
        for (counter, task) in self.task_list.iter().enumerate() {
            println!("{}. {}", counter, &task);
        }
        println!("---------------------------------");
    }

    fn add_task(&mut self, task: String) {
        self.task_list.push(Task::new(task));
    }

    pub fn add_task_prompt(&mut self) -> Result<(), Box<Error>> {
        loop {
            self.print_tasks();
            println!("Empty line returns to the menu");
            println!("Enter task to be completed");
            let mut task = String::new();
            io::stdin().read_line(&mut task)?;
            if task.trim() == "" {
                break;
            } else {
                self.add_task(task);
            }
        }
        Ok(())
    }

    pub fn remove_task(&mut self) -> Result<(), Box<Error>> {
        let task_list_len = self.task_list.len();
        if task_list_len < 1 {
            println!("There are no tasks to be removed");
        } else {
            loop {
                self.print_tasks();
                println!("Empty line returns to menu");
                println!("Enter the task number to be removed:");
                let mut task_num = String::new();
                io::stdin().read_line(&mut task_num)?;
                let task_num: usize = match task_num.trim().parse() {
                    Ok(num) if num < task_list_len => num,
                    Ok(_)   => continue,
                    Err(_)  => break,
                };
                self.task_list.remove(task_num);
            }
        }
        Ok(())
    }

    pub fn complete_task(&mut self) -> Result<(), Box<Error>> {
        let task_list_len = self.task_list.len();
        if task_list_len < 1 {
            println!("There are no tasks to be marked as completed");
        } else {
            loop {
                self.print_tasks();
                println!("Empty line returns to menu");
                println!("Enter the task number to be marked as complete:");
                let mut task_num = String::new();
                io::stdin().read_line(&mut task_num)?;
                let task_num: usize = match task_num.trim().parse() {
                    Ok(num) if num < task_list_len => num,
                    Ok(_)   => continue,
                    Err(_)  => break,
                };
                self.task_list[task_num].completed = true;
            }
        }
        Ok(())
    }
}

impl Task {
    pub fn new(command: String) -> Task {
        Task {
            completed: false,
            command: String::from(command.trim()),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.completed {
            write!(f, "[ ] - {}", self.command)
        } else {
            write!(f, "[x] - {}", self.command)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Task {
    pub completed: bool,
    pub command: String,
}

pub enum Mode {
    Add,
    Remove,
    Complete,
    Quit,
    Invalid,
}

pub struct TaskList {
    pub task_list: Vec<Task>,
}
