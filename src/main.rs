use clap::{App, AppSettings, Arg};
mod config;
mod install;

fn main() {
    // create the App with clap
    let matches = App::new("kibrarian")
        .version("0.1.0")
        .author("Carlo Supina <cdsupina@micronote.tech>")
        .about("A library manager for Kicad.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Use a custom config file")
                .takes_value(true),
        )
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
        .get_matches();

    // print config information
    let mut config = format!("{}/.config/kibrarian/config.ron", env!("HOME"));

    if let Some(c) = matches.value_of("config") {
        config = c.to_owned();
    }

    println!("Value of config: {}", config);
    config::load(config);

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

        _ => unreachable!(),
    }
}
