use std::fmt;
use std::io;
use std::string;
use std::sync;

// Aggregate all kind of different errors for SimpleUDPServer.
#[derive(Debug)]
pub struct UDPServerError<'a> {
    detail: ErrorDetail<'a>,
}

impl fmt::Display for UDPServerError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.detail)
    }
}

impl<'a> From<io::Error> for UDPServerError<'a> {
    fn from(err: io::Error) -> UDPServerError<'a> {
        UDPServerError {
            detail: ErrorDetail::IOError(err),
        }
    }
}

impl<'a> From<sync::PoisonError<sync::RwLockWriteGuard<'a, String>>>
    for UDPServerError<'a>
{
    fn from(
        err: sync::PoisonError<sync::RwLockWriteGuard<'a, String>>,
    ) -> UDPServerError {
        UDPServerError {
            detail: ErrorDetail::PoisonError(err),
        }
    }
}

impl<'a> From<string::FromUtf8Error> for UDPServerError<'a> {
    fn from(err: string::FromUtf8Error) -> UDPServerError<'a> {
        UDPServerError {
            detail: ErrorDetail::FromUtf8Error(err),
        }
    }
}

#[derive(Debug)]
enum ErrorDetail<'a> {
    IOError(io::Error),
    PoisonError(sync::PoisonError<sync::RwLockWriteGuard<'a, String>>),
    FromUtf8Error(string::FromUtf8Error),
}

impl fmt::Display for ErrorDetail<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorDetail::IOError(err) => write!(f, "{}", err.to_string()),
            ErrorDetail::PoisonError(err) => write!(f, "{}", err.to_string()),
            ErrorDetail::FromUtf8Error(err) => write!(f, "{}", err.to_string()),
        }
    }
}
