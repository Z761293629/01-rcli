use crate::process::process_csv;
use crate::CmdExecutor;

use super::verify_file;
use clap::{Args, ValueEnum};
use core::fmt;
use std::str::FromStr;

#[derive(Debug, Args)]
pub struct CsvArgs {
    #[arg(short,long,value_parser=verify_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long,value_enum,default_value_t=OutputFormat::JSON)]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

impl CmdExecutor for CsvArgs {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output.clone()
        } else {
            format!("output.{}", self.format)
        };
        process_csv(&self.input, &output, self.format)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
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
