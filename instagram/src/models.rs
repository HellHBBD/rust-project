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

    pub fn rename(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

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
    data_path: PathBuf,
}

impl Catalog {
    pub fn new(path: &PathBuf) -> Self {
        Self {
            files: Vec::new(),
            data_path: path.clone(),
        }
    }

    pub fn read(path: &PathBuf) -> Result<Self, Error> {
        let string = fs::read_to_string(&path)?;
        let list = serde_json::from_str(&string)?;
        Ok(list)
    }

    pub fn write(&self) {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        let path = self.data_path.join("catalog.json");
        fs::write(&path, json_string).expect("Fail to write file");
    }

    pub fn add(&mut self) {
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
        let file_path = self.data_path.join(file.get_file_name());

        /* store file */
        let json_string = serde_json::to_string_pretty(&temp).unwrap();
        fs::write(&file_path, json_string).expect("Fail to write file");

        /* update catalog */
        self.files.push(file);
        self.write();
    }

    pub fn rename(&mut self) {
        let path: PathBuf;
        let mut index: usize;
        loop {
            index = utils::read_usize("index of file: ");
            if index >= 0 as usize && index < self.files.len() {
                path = self.data_path.join(&self.files[index].get_file_name());
                break;
            } else {
                println!("{}", "Invalid index".red().bold());
            }
        }
        print!("new file name: ");
        io::stdout().flush().unwrap();
        let new_name = utils::read_line();
        self.files[index].rename(&new_name);
        self.write();
        let new_path = self.data_path.join(self.files[index].get_file_name());
        fs::rename(path, new_path).expect("Fail to rename file");
    }

    pub fn delete(&mut self) {
        let path: PathBuf;
        let mut index: usize;
        loop {
            index = utils::read_usize("index of file: ");
            if index >= 0 as usize
                && index < self.files.len()
                && utils::confirm(&format!(
                    "delete file \"{}\"? [y/n] ",
                    &self.files[index].get_name()
                ))
            {
                path = self.data_path.join(&self.files[index].get_file_name());
                break;
            } else {
                println!("{}", "Invalid index".red().bold());
            }
        }
        fs::remove_file(path).expect("Fail to delete file");
        self.files.remove(index);
        self.write();
    }

    pub fn compare(&self) {
        let old_path: PathBuf;
        let new_path: PathBuf;
        loop {
            let index = utils::read_usize("index of old file: ");
            if index >= 0 as usize && index < self.files.len() {
                old_path = self.data_path.join(&self.files[index].get_file_name());
                break;
            } else {
                println!("{}", "Invalid index".red().bold());
            }
        }
        loop {
            let index = utils::read_usize("index of new file: ");
            if index >= 0 as usize && index < self.files.len() {
                new_path = self.data_path.join(&self.files[index].get_file_name());
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
