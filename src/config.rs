use std::path::PathBuf;

use home::home_dir;

const QUOLL_HOME: &str = ".quoll";

/// Return the path where quoll looks at resources.
pub fn get_home() -> Option<PathBuf> {
    let mut home = home_dir()?;
    home.push(QUOLL_HOME);

    Some(home)
}
