impl Command {
    pub fn new(command: String) -> Result<Command, &'static str> {
        let command = command.trim();
        let (action, task) = command.split_at(2);
        let action = match action {
            "-a"    => Action::Add,
            "-r"    => Action::Remove,
            _       => return Err("Invalid command"),
        };
        Ok(Command {
            action: action,
            task: String::from(task),
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
