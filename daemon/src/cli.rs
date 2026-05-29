use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Start the daemon in the background
    Start,
    /// Stop the running daemon
    Stop,
    #[command(hide = true)]
    Run,
    /// Start in foreground
    Debug,
}
