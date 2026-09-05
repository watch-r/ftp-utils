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
    }
}
