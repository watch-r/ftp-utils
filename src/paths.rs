use std::{env, path::PathBuf};

pub fn home_dir() -> Result<PathBuf, &'static str> {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map(PathBuf::from)
        .map_err(|_| "Failed to determine user home directory")
}

pub fn mount_path() -> Result<PathBuf, &'static str> {
    let mut path = home_dir()?;
    path.push("Downloads");
    path.push("ISP_Media");
    Ok(path)
}

pub fn log_path() -> Result<PathBuf, &'static str> {
    let mut path = home_dir()?;
    path.push(".mntisp.log");
    Ok(path)
}
