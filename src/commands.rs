use std::fmt;
use std::path::PathBuf;

use home::home_dir;

const QUOLL_HOME: &str = ".quoll";
const DEFAULT_COMMAND_EXTENSION: &str = "png";

/// A command can tell us to quit the application (Command::Quit) or to change
/// the displayed icon using the associated String to find what we have to
/// display.
#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    Custom(String),
}

impl Command {
    /// Return the `Command` as path.
    pub fn to_path(&self) -> Option<PathBuf> {
        match self {
            Command::Quit => None,
            Command::Custom(ref c) => {
                let mut home = home_dir()?;
                home.push(QUOLL_HOME);
                home.push(c);
                home.set_extension(DEFAULT_COMMAND_EXTENSION);
                Some(home)
            }
        }
    }
}

impl<T> From<T> for Command
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        if s.as_ref() == "quit" {
            Command::Quit
        } else {
            Command::Custom(String::from(s.as_ref()))
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[test]
fn commands_creation() {
    assert_eq!(Command::from("quit"), Command::Quit);
    assert_eq!(Command::from("quit".to_string()), Command::Quit);

    let s = "eriol".to_string();
    assert_eq!(Command::from("eriol"), Command::Custom(s.clone()));
    assert_eq!(Command::from(s.clone()), Command::Custom(s));
}

#[test]
fn commands_to_path() {
    assert_eq!(Command::from("quit").to_path(), None);

    let mut home = home_dir().unwrap();
    home.push(QUOLL_HOME);
    home.push("eriol.png");
    assert_eq!(Command::from("eriol").to_path(), Some(home));
}
