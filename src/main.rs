extern crate failure;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;

use std::path::Path;
use failure::Error;
use clap::{App, Arg, AppSettings, SubCommand};
use self::config::Config;

const SUBCOMMAND_ACTION: &str = "action";

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequired)
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .subcommand(SubCommand::with_name(SUBCOMMAND_ACTION).about("start an action"))
        .get_matches();

    let config = matches.value_of("config").unwrap_or("config.toml");
    let path = Path::new(config);
    let config = Config::read_or_create(path)?;
    match matches.subcommand() {
        (SUBCOMMAND_ACTION, _) => {
            action(&config)?;
        }
        _ => {
            matches.usage();
        }
    }
    Ok(())
}

fn action(config: &Config) -> Result<(), Error> {
    info!("Message: {}", config.message);
    Ok(())
}
