use clap::{Args, Parser, Subcommand};
use core::fmt;
use std::{path::Path, str::FromStr};

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

#[derive(Debug, Args)]
pub struct GenPassArgs {
    #[arg(short, long, default_value_t = 8)]
    pub len: u8,

    #[arg(long, default_value_t = false)]
    pub no_upper: bool,

    #[arg(long, default_value_t = false)]
    pub no_lower: bool,

    #[arg(long, default_value_t = false)]
    pub no_number: bool,

    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}

#[derive(Debug, Args)]
pub struct CsvArgs {
    #[arg(short,long,value_parser=verify_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short,long,value_parser=parse_format,default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    JSON,
    YAML,
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::JSON => "json",
            OutputFormat::YAML => "yaml",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

fn parse_format(s: &str) -> Result<OutputFormat, anyhow::Error> {
    s.parse()
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::JSON),
            "yaml" => Ok(OutputFormat::YAML),
            _ => Err(anyhow::anyhow!("Invalid format!")),
        }
    }
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
