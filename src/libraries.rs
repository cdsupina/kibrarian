use crate::config::Config;
use crate::git::clone;
use fs_extra::dir;
use ron::de::from_reader;
use ron::ser;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{collections::HashMap, error, ffi::OsStr, fmt, fs, io};

#[derive(Debug)]
pub enum LibraryError {
    LibraryNotFoundError,
    LibraryInstalledError,
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryError::LibraryNotFoundError => write!(f, "Library not found."),
            LibraryError::LibraryInstalledError => write!(f, "Library already installed."),
        }
    }
}

impl error::Error for LibraryError {
    fn description(&self) -> &str {
        match self {
            LibraryError::LibraryNotFoundError => "Library not found",
            LibraryError::LibraryInstalledError => "Library already installed.",
        }
    }
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Libraries {
    lib_map: HashMap<String, Library>,
}

impl fmt::Display for Libraries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Official:")?;
        for (_, v) in self.lib_map.iter() {
            writeln!(f, "{}", v)?;
        }
        write!(f, "")
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Library {
    pub name: String,
    pub id: u64,
    pub url: String,
    pub symbols_path: String,
    pub footprints_path: String,
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
    if libraries.lib_map.contains_key(query) {
        println!("{}", libraries.lib_map[query]);
        Some(libraries.lib_map[query].clone())
    } else {
        println!("No libraries found with given query.");
        None
    }
}

pub fn install(config: Config, global: bool, query: &str) -> Result<(), Box<dyn error::Error>> {
    if let Some(library) = search(config.libraries, query) {
        // check if already installed

        let mut installed_libraries =
            match get_libraries(format!("{}/.config/kibrarian/installed.ron", env!("HOME"))) {
                Ok(x) => x,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };

        if installed_libraries.lib_map.contains_key(&library.name[..]) {
            return Err(Box::new(LibraryError::LibraryInstalledError));
        }

        let installation_path = format!("{}/.kibrarian/extra/{}", env!("HOME"), query);
        clone(&library.url[..], installation_path.clone())?;

        // copy symbol library files to symbols dir
        let library_sym_files =
            fs::read_dir(format!("{}/{}/", installation_path, library.symbols_path))?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()?;

        let library_fp_files = fs::read_dir(format!(
            "{}/{}/",
            installation_path, library.footprints_path
        ))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

        if global {
            fs::create_dir(format!(
                "{}/.kibrarian/libraries/symbols/{}",
                env!("HOME"),
                query
            ))?;
            fs::create_dir(format!(
                "{}/.kibrarian/libraries/footprints/{}",
                env!("HOME"),
                query
            ))?;
        } else {
            // TODO: create path in project directory
            unimplemented!();
        }

        for p in library_sym_files.iter() {
            let file_osstr = match p.file_name() {
                Some(x) => x,
                None => continue,
            };

            let filename = match file_osstr.to_str() {
                Some(x) => x,
                None => continue,
            };

            let mut destination = format!("{}/.kibrarian/libraries/", env!("HOME"));

            if p.extension() == Some(OsStr::new("lib")) || p.extension() == Some(OsStr::new("dcm"))
            {
                destination.push_str(&format!("symbols/{}/{}", query, filename)[..]);
            } else {
                continue;
            }
            fs::copy(p, destination)?;
        }
        for p in library_fp_files.iter() {
            let file_osstr = match p.file_name() {
                Some(x) => x,
                None => continue,
            };

            let filename = match file_osstr.to_str() {
                Some(x) => x,
                None => continue,
            };

            let mut destination = format!("{}/.kibrarian/libraries/", env!("HOME"));

            if p.extension() == Some(OsStr::new("pretty")) {
                destination.push_str(&format!("footprints/{}/{}", query, filename)[..]);
            } else {
                continue;
            }
            let mut options = dir::CopyOptions::new();
            options.copy_inside = true;
            dir::copy(p, destination, &options)?;
        }

        // create entry in installed.ron
        /*
        let mut installed_libraries =
            match get_libraries(format!("{}/.config/kibrarian/installed.ron", env!("HOME"))) {
                Ok(x) => x,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        */

        println!("adding installed library to installed.ron...");
        installed_libraries
            .lib_map
            .insert(library.name.clone(), library);
        let serialized = ser::to_string(&installed_libraries)?;

        let mut installed_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("{}/.config/kibrarian/installed.ron", env!("HOME")))
            .unwrap();

        if let Err(e) = installed_file.write(serialized.as_bytes()) {
            println!("{}", e);
        }

        Ok(())
    } else {
        Err(Box::new(LibraryError::LibraryNotFoundError))
    }
}

pub fn uninstall() -> Result<(), Box<dyn error::Error>> {
    unimplemented!();
}
