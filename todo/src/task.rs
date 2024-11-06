use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    name: String,
    time: chrono::DateTime<Local>,
}

impl Task {
    pub fn new(name: &str) -> Task {
        Task {
            name: String::from(name),
            time: Local::now(),
        }
    }

    pub fn print(&self) {
        print!("{}\t{}", self.name, self.time.format("%Y-%m-%d %H:%M:%S"));
    }
}
