use ron::de::from_reader;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};

#[derive(Debug, Deserialize)]
pub struct Libraries {
    official: HashMap<String, Library>,
    user: HashMap<String, Library>,
}

#[derive(Debug, Deserialize)]
pub struct Library {
    url: String,
    symbols_path: String,
    footprints_path: String,
}

pub fn search(library_path: String, query: &str) {
    // read libraries from file
    let f = File::open(&library_path).expect("Failed opening file.");
    let libraries: Libraries = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load libraries file: {}", e);
            std::process::exit(1);
        }
    };

    println!("Libraries: {:?}", &libraries);
}
