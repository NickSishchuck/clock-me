use clap::{Parser, Subcommand};
use clock_me::cli::CLI;

#[derive(Parser)]
#[command(name = "clock-me")]
#[command(about = "A simple CLI time tracker", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init {
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Clock in (or continue from break)
    Start,
    /// Clock out
    Stop,
    /// Take a break
    Break,
    /// Show current tracking status
    Status,
}

fn main() {
    let args = Args::parse();
    let cli = CLI::new();

    let result = match args.command {
        Commands::Init { name } => cli.handle_init(name),
        Commands::Start => cli.handle_now(),
        Commands::Stop => cli.handle_out(),
        Commands::Break => cli.handle_break(),
        Commands::Status => cli.handle_status(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
