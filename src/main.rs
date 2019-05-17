mod commands;
mod config;
mod gui;
mod network;

use std::process;
use std::sync::{Arc, RwLock};
use std::thread;

use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};
use log::{error, info};
use pretty_env_logger;

use commands::Command;
use network::udp::{send_to, Server};

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
            SubCommand::with_name("home")
                .about("Displays the location of the quoll home folder"),
        )
        .subcommand(
            SubCommand::with_name("send")
                .about("Send over UDP the specified command")
                .arg(
                    Arg::with_name("COMMAND")
                        .help("The command to send")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .about("Starts UDP server for incoming commands"),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap_or(DEFAULT_PORT);
    let address = format!("{}:{}", DEFAULT_HOST, port);

    match matches.subcommand() {
        ("home", _) => {
            if let Some(home) = config::get_home() {
                println!("{}", home.display());
            }
        }
        ("send", Some(matches)) => {
            // It's safe to unwrap COMMAND because it is required.
            let command = matches.value_of("COMMAND").unwrap();

            info!("Sending command: {}", command);

            if let Err(e) = send_to(command, address) {
                error!("{}", e);
                process::exit(1);
            }
        }
        ("serve", _) => {
            let command = Arc::new(RwLock::new(Command::from("black")));

            // Create directory to store resources if it does not exists.
            if let Some(home) = config::get_home() {
                if !home.is_dir() {
                    info!(
                        "Directory for resources not found, creating at: {}",
                        home.display()
                    );
                    if let Err(e) = config::create_home() {
                        error!("{}", e);
                        process::exit(1);
                    }
                }
            }

            info!("Starting UDP server on port {}...", port);
            let mut server = Server::new(address, Arc::clone(&command));
            thread::spawn(move || {
                if let Err(e) = server.serve_forever() {
                    error!("{}", e);
                    process::exit(1);
                };
            });

            info!("Starting GUI...");
            gui::start(command, port);
            info!("Exiting...");
        }
        _ => unreachable!(),
    }
}
