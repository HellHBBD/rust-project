use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    name: String,
    next: Option<Box<Node>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn add(&mut self, value: &str) {
        let new_node = Box::new(Node {
            name: String::from(value),
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn print(&self) {
        let mut current: &Node;
        match self.head.as_ref() {
            None => {
                println!("None");
                return;
            }
            Some(head) => current = head,
        }
        while let Some(next) = current.next.as_ref() {
            print!("{} -> ", current.name);
            current = next;
        }
        println!("{}", current.name);
    }

    pub fn write(self) {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        fs::write("data.json", json_string).unwrap();
    }

    pub fn read() -> Result<List, Error> {
        let json_string = fs::read_to_string("data.json")?;
        let list = serde_json::from_str(&json_string)?;
        Ok(list)
    }
}
