use crate::install::clone;
use ron::de::from_reader;
use serde::Deserialize;
use std::{collections::HashMap, fmt, fs::File};

pub enum LibraryError {
    LibraryNotFoundError,
    LibraryCloneError,
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryError::LibraryNotFoundError => write!(f, "Library not found."),
            LibraryError::LibraryCloneError => write!(f, "Failed to clone library."),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Libraries {
    official: HashMap<String, Library>,
}

impl fmt::Display for Libraries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Official:")?;
        for (_, v) in self.official.iter() {
            writeln!(f, "{}", v)?;
        }
        write!(f, "")
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Library {
    name: String,
    pub url: String,
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

pub fn get_libraries(library_path: String) -> Result<Libraries, ron::de::Error> {
    // read libraries from file
    let f = File::open(&library_path).expect("Failed opening file.");
    from_reader(f)
}

pub fn search(library_path: String, query: &str) -> Option<Library> {
    // read libraries from file

    let libraries = match get_libraries(library_path) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    // if the query matches the name
    if libraries.official.contains_key(query) {
        println!("{}", libraries.official[query]);
        Some(libraries.official[query].clone())
    } else {
        println!("No libraries found with given query.");
        None
    }
}

pub fn install(library_path: String, global: bool, query: &str) -> Result<(), LibraryError> {
    if let Some(library) = search(library_path, query) {
        match clone(
            &library.url[..],
            format!("{}/.kibrarian/extra/{}", env!("HOME"), query),
        ) {
            Ok(()) => Ok(()),
            Err(e) => {
                println!("{}", e);
                Err(LibraryError::LibraryCloneError)
            }
        }
    } else {
        Err(LibraryError::LibraryNotFoundError)
    }

    // TODO: if global copy libraries into ./kibrarian/libraries else copy into project libraries
    // TODO: update fp and sym tables to directory of installed library
}
