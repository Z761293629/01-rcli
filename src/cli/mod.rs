use clap::{Parser, Subcommand};
pub mod csv;
pub mod genpass;

use csv::CsvArgs;
use genpass::GenPassArgs;

#[derive(Debug, Parser)]
#[command(name="rcli",version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvArgs),

    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassArgs),
}
