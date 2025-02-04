use colored::*;
use std::collections::HashMap;
use std::io::{self, Error, Write};
use std::path::PathBuf;
use std::{fmt, fs};

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::utils::{self, parse_accounts};

pub type AccountMap = HashMap<String, i64>;

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    name: String,
    timestamp: i64,
}

impl File {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            timestamp: Local::now().timestamp(),
        }
    }

    //pub fn rename(&mut self, new_name: &str) {
    //    self.name = String::from(new_name);
    //}

    pub fn get_file_name(&self) -> String {
        format!("{}{}.json", self.name, self.timestamp)
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\t{}",
            self.name,
            utils::timestamp_to_date(self.timestamp)
        )?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalog {
    files: Vec<File>,
}

impl Catalog {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn read(path: &PathBuf) -> Result<Self, Error> {
        let string = fs::read_to_string(&path)?;
        let list = serde_json::from_str(&string)?;
        Ok(list)
    }

    pub fn write(&self, path: &PathBuf) {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        fs::write(&path, json_string).expect("Fail to write file");
    }

    pub fn add(&mut self, data_path: &PathBuf) {
        /* select file */
        print!("source path: ");
        io::stdout().flush().unwrap();
        let source_path = utils::read_line();

        /* parse source file */
        let temp = parse_accounts(&source_path);

        /* input file name */
        print!("file name: ");
        io::stdout().flush().unwrap();
        let file_name = utils::read_line();
        let file = File::new(&file_name);
        let file_path = data_path.join(file.get_file_name());

        /* store file */
        let json_string = serde_json::to_string_pretty(&temp).unwrap();
        fs::write(&file_path, json_string).expect("Fail to write file");

        /* update catalog */
        self.files.push(file);
    }

    pub fn compare(&self, data_path: &PathBuf) {
        let old_path: PathBuf;
        let new_path: PathBuf;
        loop {
            let index = utils::read_usize("index of old file: ");
            if index >= 0 as usize && index < self.files.len() {
                old_path = data_path.join(&self.files[index].get_file_name());
                break;
            } else {
                println!("{}", "Invalid index".red().bold());
            }
        }
        loop {
            let index = utils::read_usize("index of new file: ");
            if index >= 0 as usize && index < self.files.len() {
                new_path = data_path.join(&self.files[index].get_file_name());
                break;
            } else {
                println!("{}", "Invalid index".red().bold());
            }
        }
        let string = fs::read_to_string(&old_path).expect("Fail to read file");
        let old: AccountMap = serde_json::from_str(&string).expect("Fail to parse from json");
        let string = fs::read_to_string(&new_path).expect("Fail to read file");
        let new: AccountMap = serde_json::from_str(&string).expect("Fail to parse from json");
        let added: HashMap<_, _> = new
            .iter()
            .filter(|(k, _)| !old.contains_key(*k))
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        let removed: HashMap<_, _> = old
            .iter()
            .filter(|(k, _)| !new.contains_key(*k))
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        for (name, timestamp) in &added {
            println!(
                "{:30} {}",
                name.green(),
                utils::timestamp_to_date(*timestamp).green()
            );
        }
        for (name, timestamp) in &removed {
            println!(
                "{:30} {}",
                name.red(),
                utils::timestamp_to_date(*timestamp).red()
            );
        }
    }
}

impl fmt::Display for Catalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "---------------")?;
        if self.files.is_empty() {
            writeln!(f, "no data")?;
        }
        for (index, file) in self.files.iter().enumerate() {
            writeln!(f, "[{}] {}", index, file)?;
        }
        writeln!(f, "---------------")?;
        Ok(())
    }
}
