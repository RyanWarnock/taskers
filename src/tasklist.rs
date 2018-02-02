use std::error::Error;
use std::io;
use std::fs::OpenOptions;
use std::fs::DirBuilder;
use std::env;
use csv;

use task::Task;

pub struct TaskList {
    pub task_list: Vec<Task>,
    pub task_list_location: String,
}

impl TaskList {
    pub fn new() -> TaskList {
        let task_list: Vec<Task> = Vec::new();
        let task_list_location = String::from("task.list"); //default location
        TaskList { task_list, task_list_location }
    }

    pub fn get_task_file_location() -> Result<String, Box<Error>> {
        if let Some(path) = env::home_dir() {
            let path = format!("{}/.taskers", path.display());
            DirBuilder::new().recursive(true).create(&path)?;
            let cur_dir = env::current_dir()?;
            let cur_dir = format!("{}.list", cur_dir.display());
            let task_file = cur_dir.replace("/", ":");
            Ok(format!("{}/{}", path, task_file))
        } else {
            //Defaults to creating a task.list file in cwd if home_dir() not working.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tasklist_test() {
        assert_eq!(1, 1);
    }
}
