use std::fs;
use std::io;
use std::path::PathBuf;

use home::home_dir;

const QUOLL_HOME: &str = ".quoll";

/// Return the path where quoll looks at to find resources.
pub fn get_home() -> Option<PathBuf> {
    let mut home = home_dir()?;
    home.push(QUOLL_HOME);

    Some(home)
}

/// Create directory used to store resources.
pub fn create_home() -> io::Result<()> {
    if let Some(home) = get_home() {
        fs::create_dir(home)?;
    }
    Ok(())
}
