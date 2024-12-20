use clap::{Parser, Subcommand, ArgAction};

#[allow(dead_code)]
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Parser)]
#[command(author = AUTHORS, version = VERSION, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, action = ArgAction::SetTrue)]
    /// debug podcli
    pub debug: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Update
    Update,
    /// Select
    Select,
}

