use chrono::{DateTime, TimeZone, Utc};
use colored::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use crate::models::AccountMap;

pub fn parse_accounts(file_path: &str) -> AccountMap {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut accounts: AccountMap = HashMap::new();

    let mut name = String::new();
    let mut timestamp = 0;
    let mut capture_next = false;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.contains("\"href\"") {
            capture_next = true;
        } else if capture_next && line.contains("\"value\"") {
            if let Some(start) = line.find(": \"") {
                if let Some(end) = line.rfind("\"") {
                    name = line[start + 3..end].to_string();
                }
            }
        } else if capture_next && line.contains("\"timestamp\"") {
            if let Some(start) = line.find(": ") {
                timestamp = line[start + 2..].trim().parse().expect("Invalid timestamp");
            }
            accounts.insert(name.clone(), timestamp);
            capture_next = false;
        }
    }

    accounts
}

pub fn timestamp_to_date(timestamp: i64) -> String {
    let datetime: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).single().unwrap();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn read_int(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(number) => return number,
            Err(_) => println!("{}", "Please input a integer".red().bold()),
        }
    }
}

pub fn read_usize(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(number) => return number,
            Err(_) => println!("{}", "Please input a integer".red().bold()),
        }
    }
}

pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    line.trim().to_string()
}
