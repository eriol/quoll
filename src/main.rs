mod commands;
mod network;

use std::sync::{Arc, RwLock};

use log::{error, info};
use pretty_env_logger;

use commands::Command;
use network::SimpleUDPServer;

fn main() {
    pretty_env_logger::init_timed();

    let command = Arc::new(RwLock::new(Command::from("black")));
    let s = command.clone();

    info!("Starting UDP server...");
    let mut server = SimpleUDPServer::new("127.0.0.1:1234", s);
    if let Err(e) = server.serve_forever() {
        error!("{}", e);
    };
}
