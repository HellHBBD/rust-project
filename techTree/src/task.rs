use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    name: String,
    prev: HashSet<String>,
    next: HashSet<String>,
}

impl Task {
    pub fn new(n: &str) -> Self {
        Self {
            name: String::from(n),
            prev: HashSet::new(),
            next: HashSet::new(),
        }
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }
    //pub fn get_name(&self) -> String {
    //    self.name.clone()
    //}

    pub fn add_prev(&mut self, new_prev: &str) {
        self.prev.insert(String::from(new_prev));
    }
    pub fn remove_prev(&mut self, delete_prev: &str) {
        self.prev.remove(delete_prev);
    }
    //pub fn get_prev(&self) -> HashSet<String> {
    //    self.prev.clone()
    //}

    pub fn add_next(&mut self, new_next: &str) {
        self.next.insert(String::from(new_next));
    }
    pub fn remove_next(&mut self, delete_next: &str) {
        self.next.remove(delete_next);
    }
    //pub fn get_next(&self) -> HashSet<String> {
    //    self.next.clone()
    //}
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prev_tasks = self.prev.iter().cloned().collect::<Vec<_>>().join(", ");
        let next_tasks = self.next.iter().cloned().collect::<Vec<_>>().join(", ");

        writeln!(
            f,
            "---------------\ntask: {}\nprev: [{}]\nnext: [{}]\n---------------",
            self.name, prev_tasks, next_tasks
        )
    }
}
