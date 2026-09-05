use crate::paths::log_path;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

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
pub fn show_log() -> Result<(), String> {
    let path = log_path()?;
    if !path.exists() {
        println!("No logs found yet.");
        return Ok(());
    }
    let path_str = path
        .to_str()
        .ok_or_else(|| "Log path is not valid UTF-8".to_string())?;
    println!("--- Recent Activity Logs ({}) ---", path_str);
    Command::new("tail")
        .args(["-n", "15", path_str])
        .status()
        .map_err(|e| format!("Failed to run tail: {}", e))?;
    Ok(())
}
