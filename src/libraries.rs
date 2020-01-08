use ron::de::from_reader;
use serde::Deserialize;
use std::{collections::HashMap, fmt, fs::File};

#[derive(Debug, Deserialize)]
pub struct Libraries {
    official: HashMap<String, Library>,
    user: HashMap<String, Library>,
}

impl fmt::Display for Libraries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Official:")?;
        for (_, v) in self.official.iter() {
            writeln!(f, "{}", v)?;
        }
        writeln!(f, "User:")?;
        for (_, v) in self.user.iter() {
            writeln!(f, "{}", v)?;
        }
        write!(f, "")
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Library {
    name: String,
    url: String,
    symbols_path: String,
    footprints_path: String,
    installation: Option<Installation>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Installation {
    symbols_library: String,
    footprints_library: String,
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]: {}\tsyms: {}\tfps: {}",
            self.name, self.url, self.symbols_path, self.footprints_path
        )
    }
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

    //println!("Libraries: {}", &libraries);

    // if the query matches the name
    if libraries.official.contains_key(query) {
        println!("{}", libraries.official[query]);
    } else {
        println!("No libraries found with given query.");
    }
}
