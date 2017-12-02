extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate time;

use std::io;
use std::fmt;
use std::fs::OpenOptions;
use std::fs::DirBuilder;
use std::error::Error;
use std::env;

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

impl TaskList {
    pub fn new() -> TaskList {
        let task_list: Vec<Task> = Vec::new();
        let task_list_location = String::from("task.list"); //default value
        TaskList { task_list, task_list_location }
    }

    pub fn get_task_file_location() -> Result<String, Box<Error>> {
        if let Some(path) = env::home_dir() {
            let path = format!("{}/taskers", path.display());
            DirBuilder::new().recursive(true).create(&path)?;
            let cur_dir = env::current_dir()?;
            let cur_dir = format!("{}.list", cur_dir.display());
            let task_file = cur_dir.replace("/", ":");
            Ok(format!("{}/{}", path, task_file))
        } else {
            //Defaults to creating a task.list file if home_dir() not working.
            Ok(String::from("task.list"))
        }
    }

    pub fn load_from_file(&mut self) -> Result<(), Box<Error>> {
        let f = OpenOptions::new().read(true)
            .write(true)
            .create(true)
            .open(&self.task_list_location)?;
        let mut rdr = csv::Reader::from_reader(f);
        for result in rdr.deserialize() {
            let task: Task = result?;
            self.task_list.push(task);
        }
        Ok(())
    }

    pub fn save_to_file(&self) -> Result<(), Box<Error>> {
        let mut wtr = csv::Writer::from_path(&self.task_list_location)?;
        wtr.flush()?;
        for task in &self.task_list {
            wtr.serialize(&task)?;
            wtr.flush()?;
        }
        Ok(())
    }

    pub fn print_tasks(&self) {
        println!("---------------------------------");
        let task_list_len = self.task_list.len();
        if task_list_len > 0 {
            for (counter, task) in self.task_list.iter().enumerate() {
                println!("{}. {}", counter, &task);
            }
        } else {
            println!("There are no tasks");
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
        let cur_time = time::now();
        Task {
            completed: false,
            command: String::from(command.trim()),
            day: cur_time.tm_mday,
            month: cur_time.tm_mon,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.completed {
            write!(f, "[ ] {}/{} - {}", self.day, self.month, self.command)
        } else {
            write!(f, "[x] {}/{} - {}", self.day, self.month, self.command)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Task {
    pub completed: bool,
    pub command: String,
    pub day: i32,
    pub month: i32,
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
    pub task_list_location: String,
}
