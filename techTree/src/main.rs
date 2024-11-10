use std::io::{self, Write};

mod task;
mod task_list;

use crate::task_list::TaskList;

fn add_task(list: &mut TaskList) {
    loop {
        print!("task name: ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("fail to read line");
        let task_name = line.trim();
        if list.contain(task_name) {
            println!("task already exists");
            continue;
        };
        list.add_task(task_name);
        break;
    }
}

fn del_task(list: &mut TaskList) {
    loop {
        print!("task name: ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("fail to read line");
        let task_name = line.trim();
        if !list.contain(task_name) {
            println!("task not found");
            continue;
        };
        list.remove_task(task_name);
        break;
    }
}

fn select_task(list: &mut TaskList) {
    loop {
        print!("task name: ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("fail to read line");
        let task_name = line.trim();
        if !list.contain(task_name) {
            println!("task not found");
            continue;
        };
        list.select(task_name);
        break;
    }
}

fn main() {
    let mut list = match TaskList::read() {
        Ok(content) => content,
        Err(_) => TaskList::new(),
    };

    loop {
        println!("{}", list);
        print!("\n[0] = exit\n[1] = add task\n[2] = delete task\n[3] = select task\n\n(0/1/2/3): ");
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
                list.write();
                return;
            }
            1 => add_task(&mut list),
            2 => del_task(&mut list),
            3 => select_task(&mut list),
            _ => println!("invalid option"),
        }
    }
}
