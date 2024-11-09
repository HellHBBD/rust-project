use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Write};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
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
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_prev(&mut self, new_prev: &str) {
        self.prev.insert(String::from(new_prev));
    }
    pub fn remove_prev(&mut self, delete_prev: &str) {
        self.prev.remove(delete_prev);
    }
    pub fn get_prev(&self) -> HashSet<String> {
        self.prev.clone()
    }

    pub fn add_next(&mut self, new_next: &str) {
        self.next.insert(String::from(new_next));
    }
    pub fn remove_next(&mut self, delete_next: &str) {
        self.next.remove(delete_next);
    }
    pub fn get_next(&self) -> HashSet<String> {
        self.next.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskList {
    list: HashMap<String, Task>,
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, new_task: &str) {
        self.list
            .insert(String::from(new_task), Task::new(new_task));
    }
    pub fn remove_task(&mut self, delete_task: &str) {
        self.list.remove(delete_task);
        for (_, task) in &mut self.list {
            task.remove_prev(delete_task);
            task.remove_next(delete_task);
        }
    }

    pub fn connect(&mut self, task_prev: &str, task_next: &str) {
        self.list.get_mut(task_prev).unwrap().add_next(task_next);
        self.list.get_mut(task_next).unwrap().add_prev(task_prev);
    }
    pub fn unconnect(&mut self, task_prev: &str, task_next: &str) {
        self.list.get_mut(task_prev).unwrap().remove_next(task_next);
        self.list.get_mut(task_next).unwrap().remove_prev(task_prev);
    }
}

fn main() {
    //let mut list = match List::read() {
    //    Ok(content) => content,
    //    Err(_) => List::new(),
    //};

    loop {
        //list.print();
        print!("\n[0] = exit\n[1] = add name\n[2] = delete name\n\n(0/1/2): ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("fail to read line");
        let option: i32 = match line.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("please input a integer");
                continue;
            }
        };

        match option {
            0 => {
                //list.write();
                return;
            }
            1 => {
                //add_task(&mut list);
            }
            2 => {
                //del_task(&mut list);
            }
            _ => println!("invalid option"),
        }
    }
}
