impl Command {
    pub fn new(command: &str) -> Result<Command, &'static str> {
        let command = command.trim();
        if command.len() < 3 {
            return Err("Command not long enough")    
        }
        let (action, task) = command.split_at(2);
        let action = match action {
            "-a"    => Action::Add,
            "-r"    => Action::Remove,
            _       => return Err("Invalid command"),
        };
        Ok(Command {
            action: action,
            task: String::from(task.trim()),
        })
    }
}

pub enum Action {
    Add,
    Remove,
}

pub struct Command {
    pub action: Action,
    pub task: String,
}
