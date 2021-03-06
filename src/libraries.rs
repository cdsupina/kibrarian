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
    LibraryNotInstalledError,
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryError::LibraryNotFoundError => write!(f, "Library not found."),
            LibraryError::LibraryInstalledError => write!(f, "Library already installed."),
            LibraryError::LibraryNotInstalledError => write!(f, "Library is not installed."),
        }
    }
}

impl error::Error for LibraryError {
    fn description(&self) -> &str {
        match self {
            LibraryError::LibraryNotFoundError => "Library not found",
            LibraryError::LibraryInstalledError => "Library already installed.",
            LibraryError::LibraryNotInstalledError => "Library is not installed.",
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

impl Libraries {
    pub fn new() -> Libraries {
        Libraries {
            lib_map: HashMap::new(),
        }
    }
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
        // load installed libraries
        let mut installed_libraries =
            get_libraries(format!("{}/.config/kibrarian/installed.ron", env!("HOME")))?;

        // check if already installed
        if installed_libraries.lib_map.contains_key(&library.name[..]) {
            return Err(Box::new(LibraryError::LibraryInstalledError));
        }

        // clone repository
        let installation_path = format!("{}/.kibrarian/extra/{}", env!("HOME"), query);
        clone(&library.url[..], installation_path.clone())?;

        // get vector of symbol library files
        let library_sym_files =
            fs::read_dir(format!("{}/{}/", installation_path, library.symbols_path))?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()?;

        // get vector of footprint library files
        let library_fp_files = fs::read_dir(format!(
            "{}/{}/",
            installation_path, library.footprints_path
        ))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

        if global {
            // create library directories in global location
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
            // TODO: create library directory in project
            unimplemented!();
        }

        for p in library_sym_files.iter() {
            // copy lib and dcm files to symbols library directory
            let file_osstr = match p.file_name() {
                Some(x) => x,
                None => continue,
            };

            let filename = match file_osstr.to_str() {
                Some(x) => x,
                None => continue,
            };

            let mut destination = if global {
                format!("{}/.kibrarian/libraries/", env!("HOME"))
            } else {
                "./libraries/".to_owned()
            };

            if p.extension() == Some(OsStr::new("lib")) || p.extension() == Some(OsStr::new("dcm"))
            {
                destination.push_str(&format!("symbols/{}/{}", query, filename)[..]);
            } else {
                continue;
            }

            fs::copy(p, destination)?;

            // TODO: Add entry to sym-lib-table
        }

        for p in library_fp_files.iter() {
            // copy pretty directories to library directory
            let file_osstr = match p.file_name() {
                Some(x) => x,
                None => continue,
            };

            let filename = match file_osstr.to_str() {
                Some(x) => x,
                None => continue,
            };

            let mut destination = if global {
                format!("{}/.kibrarian/libraries/", env!("HOME"))
            } else {
                "./libraries/".to_owned()
            };

            if p.extension() == Some(OsStr::new("pretty")) {
                destination.push_str(&format!("footprints/{}/{}", query, filename)[..]);
            } else {
                continue;
            }

            let mut options = dir::CopyOptions::new();
            options.copy_inside = true;
            dir::copy(p, destination, &options)?;

            // TODO: Add entry to fp-lib-table
            /*
            let mut fp_lib_table_file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(config.fp_lib_table.clone())?;

            let _ = fp_lib_table_file.write(b"(test)")?;
            */
        }

        println!("Adding installed library to installed.ron...");
        installed_libraries
            .lib_map
            .insert(library.name.clone(), library);
        let serialized = ser::to_string(&installed_libraries)?;

        let mut installed_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}/.config/kibrarian/installed.ron", env!("HOME")))
            .unwrap();

        let _ = installed_file.write(serialized.as_bytes())?;

        Ok(())
    } else {
        Err(Box::new(LibraryError::LibraryNotFoundError))
    }
}

pub fn uninstall(config: Config, global: bool, query: &str) -> Result<(), Box<dyn error::Error>> {
    // check if query is in installed.ron
    if let Some(library) = search(config.libraries, query) {
        let mut installed_libraries =
            get_libraries(format!("{}/.config/kibrarian/installed.ron", env!("HOME")))?;

        if !installed_libraries.lib_map.contains_key(&library.name[..]) {
            return Err(Box::new(LibraryError::LibraryNotInstalledError));
        }

        // remove directories in .kibrarian/extra and .kibrarian/libaries
        fs::remove_dir_all(format!("{}/.kibrarian/extra/{}", env!("HOME"), query))?;
        fs::remove_dir_all(format!(
            "{}/.kibrarian/libraries/symbols/{}",
            env!("HOME"),
            query
        ))?;
        fs::remove_dir_all(format!(
            "{}/.kibrarian/libraries/footprints/{}",
            env!("HOME"),
            query
        ))?;

        // remove installed library from installed map and write to installed.ron
        installed_libraries.lib_map.remove(query).unwrap();
        let serialized = ser::to_string(&installed_libraries)?;
        let mut installed_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}/.config/kibrarian/installed.ron", env!("HOME")))
            .unwrap();

        let _ = installed_file.write(serialized.as_bytes())?;

        Ok(())
    } else {
        Err(Box::new(LibraryError::LibraryNotFoundError))
    }
}
