use crate::paths::log_path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_activity(action: &str, status: &str) -> Result<(), String> {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let path = log_path()?; // unwrap or bail out early
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| e.to_string())?; // convert io::Error -> String
    writeln!(file, "{} - {} - {}", now, action, status).map_err(|e| e.to_string())?;
    Ok(())
}
