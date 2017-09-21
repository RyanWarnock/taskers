use std::io::prelude::*;
use std::io;
use std::fs::File;
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
    Ok( match input.trim() {
        "a" => Mode::Add,
        "r" => Mode::Remove,
        "c" => Mode::Complete,
        "q" => Mode::Quit,
        _   => Mode::Invalid,
    })
}

impl TaskList {
    pub fn new() -> TaskList{
        let task_list: Vec<Task> = Vec::new();
        TaskList {
            task_list,
        }
    }

    pub fn load_from_file(&mut self) -> Result<(), Box<Error>> {
        let f = File::open("task.list");
        let mut f = match f {
            Ok(file)   => file,
            Err(err)  => {
                println!("Couldn't open file 'task.list': {}", err);
                println!("Trying to create it");
                File::create("task.list")?;
                File::open("task.list")?
            },
        };
        let mut tasks = String::new();
        f.read_to_string(&mut tasks)?;
        for line in tasks.lines() {
            self.add_task(String::from(line));    
        }
        Ok(())
    }

    pub fn save_to_file(&self) -> Result<(), Box<Error>> {
        let mut f = File::create("task.list")?;
        f.set_len(0)?;
        for task in &self.task_list {
            if task.completed == true {
                writeln!(f, "~{}", task.command)?;
            } else {
                writeln!(f, "{}", task.command)?;    
            }
        }
        Ok(())
    }

    pub fn print_tasks(&self) {
        let mut counter = 0;
        println!("---------------------------------");
        for task in &self.task_list {
            if !task.completed {
                println!("{}. [ ] - {}", counter, task.command);    
            } else {
                println!("{}. [x] - {}", counter, task.command);    
            }
            counter += 1;
        }    
        println!("---------------------------------");
    }

    pub fn add_task(&mut self, task: String) {
        let new_task = Task::new(task);
        self.task_list.push(new_task);
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
