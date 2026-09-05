use std::fs::OpenOptions;
use std::io::{self, Write};
use crate::paths::log_path;

pub fn log_activity(action: &str, status: &str) -> io::Result<()>{
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_line = format!("{} - {} - {}\n", now, action, status);

        // Open file in append mode
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path())?;

        file.write_all(log_line.as_bytes())?;
        Ok(())
    }
}
