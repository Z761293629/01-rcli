use std::str::FromStr;

use super::verify_file;
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Base64SubCommands {
    #[command(name = "encode", about = "encode string to base64")]
    Base64Encode(Base64EncodeArgs),

    #[command(name = "decode", about = "decode base64 to string")]
    Base64Decode(Base64DecodeArgs),
}

#[derive(Debug, Args)]
pub struct Base64EncodeArgs {
    #[arg(long,value_parser=verify_file,help="Input file")]
    pub input: String,

    #[arg(long,value_parser=parse_base64_format,default_value="stand",help="stand or urlsafe")]
    pub format: Base64Format,
}

#[derive(Debug, Args)]
pub struct Base64DecodeArgs {
    #[arg(long,value_parser=verify_file,help="Input file")]
    pub input: String,

    #[arg(long,value_parser=parse_base64_format,default_value="stand",help="stand or urlsafe")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Stand,
    UrlSafe,
}

fn parse_base64_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stand" => Ok(Base64Format::Stand),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("invalid format {}", s)),
        }
    }
}
