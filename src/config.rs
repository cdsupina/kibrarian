use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub libraries: String,
    pub fp_lib_table: String,
    pub sym_lib_table: String,
}

pub fn load(config_path: String) -> Config {
    let f = File::open(&config_path).expect("Failed opening file.");
    let config: Config = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    println!("Config: {:?}", &config);
    config
}
