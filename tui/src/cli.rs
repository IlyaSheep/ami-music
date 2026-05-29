use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Daemon IP address (without port) to connect to
    #[arg(short = 'a', long = "address")]
    pub address: String,
}
