mod commands;
mod gui;
mod network;

use std::sync::{Arc, RwLock};
use std::thread;

use log::{error, info};
use pretty_env_logger;

use commands::Command;
use network::SimpleUDPServer;

fn main() {
    pretty_env_logger::init_timed();

    let command = Arc::new(RwLock::new(Command::from("black")));

    info!("Starting UDP server...");
    let mut server =
        SimpleUDPServer::new("127.0.0.1:1234", Arc::clone(&command));
    thread::spawn(move || {
        if let Err(e) = server.serve_forever() {
            error!("{}", e);
        };
    });

    info!("Starting GUI...");
    gui::start(command);
    info!("Exiting...");
}
