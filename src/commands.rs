// A command can tell us to quit the application (Command::Quit) or to change
// the displayed icon using the associated String to find what we have to
// display.
#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    Custom(String),
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

#[test]
fn commands_conversion() {
    assert_eq!(Command::from("quit"), Command::Quit);
    assert_eq!(Command::from("quit".to_string()), Command::Quit);

    let s = "eriol".to_string();
    assert_eq!(Command::from("eriol"), Command::Custom(s.clone()));
    assert_eq!(Command::from(s.clone()), Command::Custom(s));
}
