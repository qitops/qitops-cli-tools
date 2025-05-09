use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "qitops")]
#[command(about = "Software Quality Assurance CLI for API, Performance, Security, and Web Testing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// API testing commands
    Api {},
    /// Performance testing commands
    Perf {},
    /// Security testing commands
    Sec {},
    /// Web testing commands
    Web {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Api {} => println!("API testing selected"),
        Commands::Perf {} => println!("Performance testing selected"),
        Commands::Sec {} => println!("Security testing selected"),
        Commands::Web {} => println!("Web testing selected"),
    }
}
