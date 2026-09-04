use clap::{Parser, Subcommand};

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
        Commands::Start => println!("You picked Start"),
        Commands::Stop => println!("You picked Start"),
    }
}
