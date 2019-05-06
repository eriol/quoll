mod network;

use std::sync::{Arc, RwLock};

use log::{error, info};
use pretty_env_logger;

use network::SimpleUDPServer;

fn main() {
    pretty_env_logger::init_timed();

    let shared_state = Arc::new(RwLock::new(String::new()));
    let s = shared_state.clone();

    info!("Starting UDP server...");
    let mut server = SimpleUDPServer::new("127.0.0.1:1234", s);
    if let Err(e) = server.serve_forever() {
        error!("{}", e);
    };
}
