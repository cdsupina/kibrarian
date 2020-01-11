use clap::{App, AppSettings, Arg};
mod config;
mod git;
mod libraries;

fn main() {
    // create the App with clap
    let matches = App::new("kibrarian")
        .version("0.1.0")
        .author("Carlo Supina <cdsupina@micronote.tech>")
        .about("A library manager for Kicad.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("install")
                .about("Installs a library.")
                .arg(
                    Arg::with_name("global")
                        .help("Indicate global.")
                        .short("g")
                        .long("global"),
                )
                .arg(
                    Arg::with_name("target")
                        .help("Target library to install.")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("uninstall")
                .about("Uninstalls a library.")
                .arg(
                    Arg::with_name("global")
                        .help("Indicate global.")
                        .short("g")
                        .long("global"),
                )
                .arg(
                    Arg::with_name("target")
                        .help("Target library to uninstall.")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("search").about("Search for a library.").arg(
                Arg::with_name("query")
                    .help("Query to search.")
                    .index(1)
                    .required(true),
            ),
        )
        .subcommand(App::new("update").about("Update libraries."))
        .subcommand(App::new("setup").about("Setup kibrarian configuration."))
        .get_matches();

    // create config.ron path
    let config_path = format!("{}/.config/kibrarian/config.ron", env!("HOME"));

    if let Some(config_file) = config::load(&config_path[..]) {
        // handle subcommands and args
        match matches.subcommand() {
            ("install", Some(install_matches)) => {
                println!(
                    "Installing {}...",
                    install_matches.value_of("target").unwrap()
                );

                match libraries::install(
                    config_file,
                    install_matches.is_present("global"),
                    install_matches.value_of("target").unwrap(),
                ) {
                    Ok(()) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            ("uninstall", Some(uninstall_matches)) => {
                println!(
                    "Uninstalling {}...",
                    uninstall_matches.value_of("target").unwrap()
                );

                match libraries::uninstall() {
                    Ok(()) => {}
                    Err(_) => std::process::exit(1),
                }
            }

            ("search", Some(search_matches)) => {
                match libraries::search(
                    config_file.libraries,
                    search_matches.value_of("query").unwrap(),
                ) {
                    Some(x) => x,
                    None => {
                        std::process::exit(1);
                    }
                };
            }

            ("setup", Some(_)) => {
                config::setup(Some(config_file));
            }

            ("update", Some(_)) => {
                println!("Updating library sources");
            }

            _ => unreachable!(),
        }
    } else {
        match matches.subcommand() {
            ("setup", Some(_)) => {
                config::setup(None);
            }

            _ => println!(
                "No config file found. Run 'kibrarian setup' if you are a first time user."
            ),
        }
    }
}
