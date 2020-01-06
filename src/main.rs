use clap::{App, AppSettings, Arg};
mod install;

//static GLOBAL_FP_LIB_TABLE: &str = ".test/config/kicad/fp-lib-table";
//static GLOBAL_SYM_LIB_TABLE: &str = ".test/config/kicad/sym-lib-table";

fn main() {
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
        .get_matches();

    // global install
    // install libraries to KICAD_LIBRARY_DIR
    // update GLOBAL_FP_LIB_TABLE and SYM_LIB_TABLE
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
