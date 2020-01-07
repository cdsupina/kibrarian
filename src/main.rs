use clap::{App, AppSettings, Arg};
use std::path::Path;
mod config;
mod install;
mod libraries;

fn main() {
    // create the App with clap
    let matches = App::new("kibrarian")
        .version("0.1.0")
        .author("Carlo Supina <cdsupina@micronote.tech>")
        .about("A library manager for Kicad.")
        .setting(AppSettings::ArgRequiredElseHelp)
        /*
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Use a custom config file")
                .takes_value(true),
        )
        */
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
        .subcommand(App::new("setup").about("Setup kibrarian configuration."))
        .get_matches();

    // print config information
    let config_path = format!("{}/.config/kibrarian/config.ron", env!("HOME"));

    /*
    if let Some(c) = matches.value_of("config") {
        config_path = c.to_owned();
    }
    */

    // if config.ron does not exist at given path run prompt to run setup
    /*
    if !Path::new(&config_path[..]).exists() {
        println!("No config file found. Run 'kibrarian setup' if you are a first time user.\nAlternatively specify a valid config.ron.");
        std::process::exit(1);
    } else {
        //println!("Config file path: {}", config_path);
        let config_file = config::load(config_path);
    }
    */
    if let Some(config_file) = config::load(&config_path[..]) {
        // handle subcommands and args
        match matches.subcommand() {
            ("install", Some(install_matches)) => {
                println!(
                    "Installing {} ",
                    install_matches.value_of("target").unwrap()
                );
                if install_matches.is_present("global") {
                    println!("installing globally");
                } else {
                    println!("installing for project");
                }

                match install::install(
                    install_matches.is_present("global"),
                    install_matches.value_of("target").unwrap(),
                ) {
                    Ok(()) => {}
                    Err(e) => println!("error: {}", e),
                }
            }

            ("uninstall", Some(uninstall_matches)) => {
                println!(
                    "Uninstalling {}",
                    uninstall_matches.value_of("target").unwrap()
                );
                if uninstall_matches.is_present("global") {
                    println!("uninstalling globally");
                } else {
                    println!("uninstalling for project");
                }
            }

            ("search", Some(search_matches)) => libraries::search(
                config_file.libraries,
                search_matches.value_of("query").unwrap(),
            ),

            ("setup", Some(setup_matches)) => {
                println!("Running setup for kibrarian.");
                config::setup(&config_path[..]);
            }

            _ => unreachable!(),
        }
    } else {
        match matches.subcommand() {
            ("setup", Some(setup_matches)) => {
                println!("Running setup for kibrarian.");
                config::setup(&config_path[..]);
            }

            _ => println!(
                "No config file found. Run 'kibrarian setup' if you are a first time user."
            ),
        }
    }
}
