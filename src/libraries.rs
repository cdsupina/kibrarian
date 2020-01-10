use crate::git::clone;
use fs_extra::copy_items_with_progress;
use ron::de::from_reader;
use serde::Deserialize;
use std::{collections::HashMap, error, ffi::OsStr, fmt, fs, io};

#[derive(Debug)]
pub enum LibraryError {
    LibraryNotFoundError,
    LibraryCloneError,
    LibraryCopyError,
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryError::LibraryNotFoundError => write!(f, "Library not found."),
            LibraryError::LibraryCloneError => write!(f, "Failed to clone library."),
            LibraryError::LibraryCopyError => write!(f, "Failed to copy library."),
        }
    }
}

impl error::Error for LibraryError {
    fn description(&self) -> &str {
        match self {
            LibraryError::LibraryNotFoundError => "Library not found",
            LibraryError::LibraryCloneError => "Failed to clone library",
            LibraryError::LibraryCopyError => "Failed to copy library",
        }
    }
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
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
    pub symbols_path: String,
    pub footprints_path: String,
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
    let f = fs::File::open(&library_path).expect("Failed opening file.");
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

pub fn install(
    library_path: String,
    global: bool,
    query: &str,
) -> Result<(), Box<dyn error::Error>> {
    if let Some(library) = search(library_path, query) {
        clone(
            &library.url[..],
            format!("{}/.kibrarian/extra/{}", env!("HOME"), query),
        )?;

        let library_sym_files = fs::read_dir(format!(
            "{}/.kibrarian/extra/{}/{}/",
            env!("HOME"),
            query,
            library.symbols_path
        ))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

        fs::create_dir(format!(
            "{}/.kibrarian/libraries/symbols/{}",
            env!("HOME"),
            query
        ))?;

        for p in library_sym_files.iter() {
            if p.extension() == Some(OsStr::new("lib")) || p.extension() == Some(OsStr::new("dcm"))
            {
                let filename = match p.file_name() {
                    Some(x) => x,
                    None => continue,
                };
                fs::copy(
                    p,
                    format!(
                        "{}/.kibrarian/libraries/symbols/{}/{}",
                        env!("HOME"),
                        query,
                        match filename.to_str() {
                            Some(x) => x,
                            None => continue,
                        },
                    ),
                )?;
            }
        }
        Ok(())
    } else {
        Err(Box::new(LibraryError::LibraryNotFoundError))
    }
}
