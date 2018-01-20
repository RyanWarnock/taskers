extern crate time;

use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Task {
    pub completed: bool,
    pub command: String,
    pub day: i32,
    pub month: i32,
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
