use colored::*;
use dirs::data_dir;
//use std::collections::HashMap;
use std::fs;

mod models;
mod utils;

use models::Catalog;

fn main() {
    /* initialize data path */
    let mut data_path = data_dir().expect("Could not determine data directory.");
    data_path.push("instagram");
    fs::create_dir_all(&data_path).expect("Failed to create directory");

    /* read catalog */
    let catalog_path = data_path.join("catalog.json");
    let mut catalog = match Catalog::read(&catalog_path) {
        Ok(c) => c,
        Err(_) => Catalog::new(),
    };

    /* option */
    loop {
        println!("{}", catalog);
        let option = utils::read_int("\n[0] = exit\n[1] = add new file\n[2] = rename file\n[3] = delete file\n[4] = compare files\n\n(0/1/2/3/4): ");
        match option {
            /* exit */
            0 => {
                catalog.write(&catalog_path);
                return;
            }
            /* add */
            1 => {
                catalog.add(&data_path);
            }
            /* rename */
            2 => {
                println!("2")
            }
            /* delete */
            3 => {
                println!("3")
            }
            /* compare */
            4 => {
                catalog.compare(&data_path);
            }
            _ => println!("{}", "Invalid option".red().bold()),
        }
    }
}
