use clap::{Parser, Subcommand};
use clockme::cli::CLI;

#[derive(Parser)]
#[command(name = "clock-me")]
#[command(about = "A simple CLI time tracker", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new time tracking project
    Init {
        /// Name of the project
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Clock in and start tracking time
    Now,
    /// Clock out and stop tracking time
    Out,
    /// Show current tracking status
    Status,
}

fn main() {
    let args = Args::parse();
    let cli = CLI::new();

    let result = match args.command {
        Commands::Init { name } => cli.handle_init(name),
        Commands::Now => cli.handle_now(),
        Commands::Out => cli.handle_out(),
        Commands::Status => cli.handle_status(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
