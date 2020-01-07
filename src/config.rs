use ron::de::from_reader;
use serde::Deserialize;
use std::{fmt, fs::File, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub libraries: String,
    pub fp_lib_table: String,
    pub sym_lib_table: String,
}

impl Config {
    pub fn new() -> Config {
        unimplemented!();
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Libraries path: {}", self.libraries)?;
        writeln!(f, "fp-lib-table: {}", self.fp_lib_table)?;
        writeln!(f, "sym-lib-table: {}", self.sym_lib_table)
    }
}

pub fn setup(config_path: &str) {
    println!("Kibrarian Setup");

    // check if config.ron exists
    if Path::new(config_path).exists() {
        // display current config
        println!("config.ron file found.");
    } else {
        println!("config.ron file not found.");
    }
}

pub fn load(config_path: &str) -> Option<Config> {
    // let f = File::open(&config_path).expect("Failed opening file.");
    let f: File = match File::open(config_path) {
        Ok(x) => x,
        Err(_) => {
            return None;
        }
    };
    let config: Option<Config> = match from_reader(f) {
        Ok(x) => Some(x),
        Err(e) => {
            println!("config.ron not correct: {}", e);
            None
        }
    };

    //println!("Config: {:?}", &config);
    config
}
