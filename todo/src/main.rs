pub mod list;
pub mod task;

use crate::list::List;
use std::io::{self, Write};

fn add_task(list: &mut List) {
    print!("task name: ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("fail to read line");
    list.add(line.trim());
}

fn del_task(list: &mut List) {
    loop {
        print!("index: ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("fail to read line");
        let index: usize = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a integer");
                continue;
            }
        };
        if index >= list.len() {
            println!("out of index");
            continue;
        }
        list.delete(index);
        break;
    }
}

fn main() {
    let mut list = match List::read() {
        Ok(content) => content,
        Err(_) => List::new(),
    };

    loop {
        list.print();
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
                list.write();
                return;
            }
            1 => {
                add_task(&mut list);
            }
            2 => {
                del_task(&mut list);
            }
            _ => println!("invalid option"),
        }
    }
}
