use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::{self, Error, Write};

use serde::{Deserialize, Serialize};

//mod task;
use crate::task::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    list: HashMap<String, Task>,
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }
    //pub fn len(&self) -> usize{
    //    self.list.len()
    //}
    pub fn contain(&self, task: &str) -> bool {
        self.list.contains_key(task)
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

    fn _rename(&mut self, task_name: &str) -> String {
        loop {
            print!("new name: ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("fail to read line");
            let new_name = line.trim();
            if self.contain(new_name) {
                println!("task name should be unique");
                continue;
            }
            let current_task = self.list.get_mut(task_name).unwrap();
            current_task.set_name(&new_name);
            if let Some(value) = self.list.remove(task_name) {
                self.list.insert(String::from(new_name), value);
            }
            return String::from(new_name);
        }
    }
    fn _add_prev(&mut self, current_task: &str) {
        loop {
            println!("{}", self);
            print!("prev task name: ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("fail to read line");
            let prev_task = line.trim();
            if !self.contain(prev_task) {
                println!("task name not found");
                continue;
            }
            self.connect(prev_task, current_task);
            break;
        }
    }
    fn _add_next(&mut self, current_task: &str) {
        loop {
            println!("{}", self);
            print!("next task name: ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("fail to read line");
            let next_task = line.trim();
            if !self.contain(next_task) {
                println!("task name not found");
                continue;
            }
            self.connect(current_task, next_task);
            break;
        }
    }
    fn _unconnect(&mut self, current_task: &str) {
        loop {
            println!("{}", self.list.get(current_task).unwrap());
            print!("task name: ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("fail to read line");
            let other_task = line.trim();
            if !self.contain(other_task) {
                println!("task name not found");
                continue;
            }
            self.unconnect(other_task, current_task);
            self.unconnect(current_task, other_task);
            break;
        }
    }
    pub fn select(&mut self, task_name: &str) {
        let mut current_task = String::from(task_name);
        loop {
            println!("{}", self.list.get(&current_task).unwrap());
            print!("\n[0] = exit\n[1] = rename\n[2] = add prev\n[3] = add next\n[4] = unconnect\n\n(0/1/2/3/4): ");
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
                    return;
                }
                1 => current_task = self._rename(&current_task),
                2 => self._add_prev(&current_task),
                3 => self._add_next(&current_task),
                4 => self._unconnect(&current_task),
                _ => println!("invalid option"),
            }
        }
    }

    pub fn write(&self) {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        fs::write("data.json", json_string).expect("fail to write file");
    }
    pub fn read() -> Result<TaskList, Error> {
        let string = fs::read_to_string("data.json")?;
        let list = serde_json::from_str(&string)?;
        Ok(list)
    }
}

impl fmt::Display for TaskList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "---------------")?;
        if self.list.is_empty() {
            write!(f, "empty task list")?;
        }
        for (name, _) in &self.list {
            writeln!(f, "[ ] {}", name)?;
        }
        writeln!(f, "---------------")?;
        Ok(())
    }
}
