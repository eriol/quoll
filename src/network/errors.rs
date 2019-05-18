use std::fmt;
use std::io;
use std::string;
use std::sync;

use crate::commands::Command;

// Aggregate all kind of different errors for udp::Server.
#[derive(Debug)]
pub enum UDPServerError<'a> {
    IOError(io::Error),
    PoisonError(sync::PoisonError<sync::RwLockWriteGuard<'a, Command>>),
    FromUtf8Error(string::FromUtf8Error),
}

impl fmt::Display for UDPServerError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UDPServerError::IOError(err) => write!(f, "{}", err.to_string()),
            UDPServerError::PoisonError(err) => {
                write!(f, "{}", err.to_string())
            }
            UDPServerError::FromUtf8Error(err) => {
                write!(f, "{}", err.to_string())
            }
        }
    }
}

impl<'a> From<io::Error> for UDPServerError<'a> {
    fn from(err: io::Error) -> UDPServerError<'a> {
        UDPServerError::IOError(err)
    }
}

impl<'a> From<sync::PoisonError<sync::RwLockWriteGuard<'a, Command>>>
    for UDPServerError<'a>
{
    fn from(
        err: sync::PoisonError<sync::RwLockWriteGuard<'a, Command>>,
    ) -> UDPServerError {
        UDPServerError::PoisonError(err)
    }
}

impl<'a> From<string::FromUtf8Error> for UDPServerError<'a> {
    fn from(err: string::FromUtf8Error) -> UDPServerError<'a> {
        UDPServerError::FromUtf8Error(err)
    }
}
