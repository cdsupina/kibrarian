use ron::de::from_reader;
use ron::ser;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{error, fmt, fs, fs::File, io};

#[derive(Debug, Deserialize, Serialize)]
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

    pub fn libraries(&mut self, path: &str) {
        self.libraries = path.to_owned();
    }

    pub fn installed(&mut self, path: &str) {
        self.installed = path.to_owned();
    }

    pub fn fp_lib_table(&mut self, path: &str) {
        self.fp_lib_table = path.to_owned();
    }

    pub fn sym_lib_table(&mut self, path: &str) {
        self.sym_lib_table = path.to_owned();
    }

    pub fn wizard(&mut self) {
        println!("Welcome to the Kibrarian Setup Wizard!");

        // libraries.ron
        println!("libraries.ron Path:");
        println!(
            "Press ENTER to use default: '{}/.config/kibrarian/libraries.ron' or enter a custom path.",
            env!("HOME")
        );
        let mut libraries_path = String::new();
        io::stdin()
            .read_line(&mut libraries_path)
            .expect("Couldn't read line.");
        let len = libraries_path.trim_end_matches(&['\r', '\n'][..]).len();
        libraries_path.truncate(len);
        if libraries_path != "" {
            self.libraries(&libraries_path[..]);
        }

        // installed.ron
        println!("installed.ron Path:");
        println!(
            "Press ENTER to use default: '{}/.config/kibrarian/installed.ron' or enter a custom path.",
            env!("HOME")
        );
        let mut installed_path = String::new();
        io::stdin()
            .read_line(&mut installed_path)
            .expect("Couldn't read line.");
        let len = installed_path.trim_end_matches(&['\r', '\n'][..]).len();
        installed_path.truncate(len);
        if installed_path != "" {
            self.installed(&installed_path[..]);
        }

        // fp_lib_table
        println!("fp_lib_table Path:");
        println!(
            "Press ENTER to use default: '{}/projects/kibrarian/test/config/kicad/fp-lib-table' or enter a custom path.",
            env!("HOME")
        );
        let mut fp_lib_table_path = String::new();
        io::stdin()
            .read_line(&mut fp_lib_table_path)
            .expect("Couldn't read line.");
        let len = fp_lib_table_path.trim_end_matches(&['\r', '\n'][..]).len();
        fp_lib_table_path.truncate(len);
        if fp_lib_table_path != "" {
            self.fp_lib_table(&fp_lib_table_path[..]);
        }

        // sym_lib_table
        println!("sym_lib_table Path:");
        println!(
            "Press ENTER to use default: '{}/projects/kibrarian/test/config/kicad/sym-lib-table' or enter a custom path.",
            env!("HOME")
        );
        let mut sym_lib_table_path = String::new();
        io::stdin()
            .read_line(&mut sym_lib_table_path)
            .expect("Couldn't read line.");
        let len = sym_lib_table_path.trim_end_matches(&['\r', '\n'][..]).len();
        sym_lib_table_path.truncate(len);
        if sym_lib_table_path != "" {
            self.sym_lib_table(&sym_lib_table_path[..]);
        }

        println!("{}", self);
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "libraries.ron path: {}", self.libraries)?;
        writeln!(f, "installed.ron path: {}", self.installed)?;
        writeln!(f, "fp-lib-table: {}", self.fp_lib_table)?;
        write!(f, "sym-lib-table: {}", self.sym_lib_table)
    }
}

pub fn setup(config_file: Option<Config>) -> Result<(), Box<dyn error::Error>> {
    println!("Kibrarian Setup");

    if let Some(c) = config_file {
        println!("config.ron file found.\n{}", c);
    } else {
        println!("config.ron file not found.");
        let mut new_config = Config::new();
        new_config.wizard();

        // write to file
        let serialized = ser::to_string(&new_config)?;

        let mut new_config_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}/.config/kibrarian/config.ron", env!("HOME")))
            .unwrap();

        let _ = new_config_file.write(serialized.as_bytes())?;
    }
    Ok(())
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
