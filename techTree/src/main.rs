use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct People {
    name: String,
    age: i32,
}

impl People {
    pub fn new(n: &str, a: i32) -> Self {
        Self {
            name: String::from(n),
            age: a,
        }
    }
    pub fn change_age(&mut self, new_age: i32) {
        self.age = new_age;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    prev: Vec<Task>,
    next: Vec<Task>,
}

impl Task {
    pub fn new() {}
}

fn main() {
    let mut map = HashMap::new();
    map.insert("paul", People::new("paul", 19));
    let me: &mut People = map.get_mut("paul").unwrap();
    println!("{:?}", me);
    me.change_age(20);
    println!("{:?}", me);
}
