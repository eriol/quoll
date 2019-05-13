mod commands;
mod gui;
mod network;

use std::sync::{Arc, RwLock};
use std::thread;

use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};
use log::{error, info};
use pretty_env_logger;

use commands::Command;
use network::SimpleUDPServer;

const ABOUT: &str = "
Display custom icons on system tray.
Supported format for icons are SVG and PNG.";
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "1738";

fn main() {
    pretty_env_logger::init_timed();

    let matches = App::new("quoll")
        .about(ABOUT)
        .author(crate_authors!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .value_name("PORT"),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .about("Starts UDP server for incoming commands"),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap_or(DEFAULT_PORT);

    if let Some(_) = matches.subcommand_matches("serve") {
        let command = Arc::new(RwLock::new(Command::from("black")));

        info!("Starting UDP server on port {}...", port);
        let mut server = SimpleUDPServer::new(
            format!("{}:{}", DEFAULT_HOST, port),
            Arc::clone(&command),
        );
        thread::spawn(move || {
            if let Err(e) = server.serve_forever() {
                error!("{}", e);
            };
        });

        info!("Starting GUI...");
        gui::start(command);
        info!("Exiting...");
    }
}
