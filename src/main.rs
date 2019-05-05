mod network;

use std::sync::{Arc, RwLock};

use network::SimpleUDPServer;

fn main() {
    let shared_state = Arc::new(RwLock::new(String::new()));
    let s = shared_state.clone();

    let mut server = SimpleUDPServer::new("127.0.0.1:1234", s);
    if let Err(e) = server.serve_forever() {
        println!("{}", e);
    };
}
