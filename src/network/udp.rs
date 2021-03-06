use std::io;
use std::net::UdpSocket;
use std::result;
use std::sync::{Arc, RwLock};

use log::debug;

use super::errors::UDPServerError;
use crate::commands::Command;

// Maximum buffer size for commands.
//
// Traditionally (so excluding neologisms) the longest word in Italian is
// precipitevolissimevolmente, made by 26 letters. For our use case is more
// than enough.
const MAX_BUF_SIZE: usize = 26;

pub type Result<'a, T> = result::Result<T, UDPServerError<'a>>;

/// A simple server listening on UDP.
///
/// The server receives commands as String, with a max len of MAX_BUF_SIZE.
/// Commands that exceed MAX_BUF_SIZE are simply truncated.
pub struct Server {
    address: String,
    command: Arc<RwLock<Command>>,
    socket: Option<io::Result<UdpSocket>>,
}

impl Server {
    /// Create a new server.
    pub fn new(address: String, command: Arc<RwLock<Command>>) -> Self {
        Server {
            address,
            command,
            socket: None,
        }
    }

    fn bind(&mut self) {
        self.socket = Some(UdpSocket::bind(&self.address));
    }

    fn recv_command(&self) -> Result<()> {
        let mut buf = [0; MAX_BUF_SIZE];

        match self.socket.as_ref() {
            Some(socket) => match socket {
                Ok(socket) => {
                    let (bytes, src) = socket.recv_from(&mut buf)?;
                    let mut command = self.command.write()?;
                    *command = Command::from(String::from_utf8(
                        buf[..bytes].to_vec(),
                    )?);

                    debug!("Received command '{}' from {}", *command, src);

                    Ok(())
                }
                // io::Error doesn't support Clone trait so we create a new one.
                Err(e) => Err(UDPServerError::from(io::Error::new(
                    e.kind(),
                    format!(
                        "Error on binding to {}: {}",
                        &self.address,
                        e.to_string()
                    ),
                ))),
            },
            None => panic!("SimpleUDPServer::bind must be called!"),
        }
    }

    /// Listen forever for incoming commands.
    pub fn serve_forever(&mut self) -> Result<()> {
        self.bind();
        loop {
            self.recv_command()?
        }
    }
}

/// Send over UDP the specified command to the specified address.
pub fn send_to(command: &str, address: String) -> io::Result<usize> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let bytes = socket.send_to(command.as_bytes(), address)?;
    Ok(bytes)
}
