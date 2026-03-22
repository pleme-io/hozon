use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hozon", about = "Encrypted device backup")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Back up a device
    Backup,
    /// Restore a device from backup
    Restore,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Backup => {
            println!("hozon: starting backup");
        }
        Commands::Restore => {
            println!("hozon: starting restore");
        }
    }
}
