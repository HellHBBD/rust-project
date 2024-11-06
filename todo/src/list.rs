use crate::task::Task;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    list: Vec<Task>,
}

impl List {
    pub fn new() -> List {
        List { list: Vec::new() }
    }

    pub fn add(&mut self, name: &str) {
        self.list.push(Task::new(name));
    }

    pub fn delete(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn print(&self) {
        println!("-------------------------");
        if self.list.len() == 0 {
            println!("empty list");
        }
        for (index, task) in self.list.iter().enumerate() {
            print!("[{}] ", index);
            task.print();
            println!();
        }
        println!("-------------------------");
    }

    pub fn write(&self) {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        fs::write("data.json", json_string).expect("fail to write file");
    }

    pub fn read() -> Result<List, Error> {
        let string = fs::read_to_string("data.json")?;
        let list = serde_json::from_str(&string)?;
        Ok(list)
    }
}
