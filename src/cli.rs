use clap::{Args, Parser, Subcommand};
use std::path::Path;

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
}

#[derive(Debug, Args)]
pub struct CsvArgs {
    #[arg(short,long,value_parser=verify_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
