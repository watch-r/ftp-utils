use clap::{Parser, Subcommand};

mod logging;
mod mount;
mod paths;

#[derive(Parser)]
#[command(name = "mntisp")]
#[command(about = "Manages the rclone ISP Media mount")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mounts the ISP Media share
    Start,
    /// Unmounts the ISP Media share
    Stop,
    /// Checks if the directory is currently mounted
    Status,
    /// Shows the last 15 lines of activity logs
    Log,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => {
            if let Err(e) = mount::start() {
                eprintln!("Error during start: {}", e);
            }
        }
        Commands::Stop => {
            if let Err(e) = mount::stop() {
                eprintln!("Error during stop: {}", e);
            }
        }
        Commands::Status => {
            if let Err(e) = mount::status() {
                eprintln!("Error checking status: {}", e);
            }
        }
        Commands::Log => {
            if let Err(e) = logging::show_log() {
                eprintln!("Error showing log: {}", e);
            }
        }
    }
}
