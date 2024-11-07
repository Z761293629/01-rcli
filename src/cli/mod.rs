pub mod base64;
pub mod csv;
pub mod genpass;
pub mod http;
pub mod text;

pub use base64::Base64SubCommands;
use clap::{Parser, Subcommand};
use csv::CsvArgs;
use genpass::GenPassArgs;
use http::HttpSubCommands;
use std::path::{Path, PathBuf};
use text::TextSubCommands;

use crate::CmdExecutor;

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

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassArgs),

    #[command(subcommand, about = "Base64 encode or decode")]
    Base64(Base64SubCommands),

    #[command(subcommand, about = "Sign or verify text")]
    Text(TextSubCommands),

    #[command(subcommand)]
    Http(HttpSubCommands),
}

impl CmdExecutor for SubCommands {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommands::Csv(args) => args.execute().await,
            SubCommands::GenPass(args) => args.execute().await,
            SubCommands::Base64(cmd) => cmd.execute().await,
            SubCommands::Text(cmd) => cmd.execute().await,
            SubCommands::Http(cmd) => cmd.execute().await,
        }
    }
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename.eq("-") || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("directory does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file() {
        assert!(verify_file("-").is_ok());
        assert!(verify_file("Cargo.toml").is_ok());
        assert_eq!(verify_file("not-exist-file"), Err("File does not exist"));
    }
}
