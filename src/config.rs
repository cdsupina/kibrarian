use ron::de::from_reader;
use serde::Deserialize;
use std::{fmt, fs::File};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub libraries: String,
    pub installed: String,
    pub fp_lib_table: String,
    pub sym_lib_table: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            libraries: format!("{}/.config/kibrarian/libraries.ron", env!("HOME")),
            installed: format!("{}/.config/kibrarian/installed.ron", env!("HOME")),
            fp_lib_table: format!(
                "{}/projects/kibrarian/test/config/kicad/fp-lib-table",
                env!("HOME")
            ),
            sym_lib_table: format!(
                "{}/projects/kibrarian/test/config/kicad/sym-lib-table",
                env!("HOME")
            ),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Libraries path: {}", self.libraries)?;
        writeln!(f, "fp-lib-table: {}", self.fp_lib_table)?;
        write!(f, "sym-lib-table: {}", self.sym_lib_table)
    }
}

pub fn setup(config_file: Option<Config>) {
    println!("Kibrarian Setup");

    if let Some(c) = config_file {
        println!("config..ron file found.\n{}", c);
    } else {
        println!("config.ron file not found.");
    }
}

pub fn load(config_path: &str) -> Option<Config> {
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

    config
}
