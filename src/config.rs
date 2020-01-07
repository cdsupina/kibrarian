use ron::de::from_reader;
use serde::Deserialize;
use std::{fmt, fs::File};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub libraries: String,
    pub fp_lib_table: String,
    pub sym_lib_table: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            libraries: format!("{}/.kibrarian/libraries", env!("HOME")),
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

    pub fn libraries(&mut self, path: &str) {
        self.libraries = path.to_owned();
    }

    pub fn fp_lib_table(&mut self, path: &str) {
        self.fp_lib_table = path.to_owned();
    }

    pub fn sym_lib_table(&mut self, path: &str) {
        self.sym_lib_table = path.to_owned();
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
