use std::str::FromStr;

use crate::{
    process::{base64_decode, base64_encode},
    utils::get_reader,
    CmdExecutor,
};

use super::verify_file;
use clap::{Args, Subcommand, ValueEnum};
use enum_dispatch::enum_dispatch;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
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

    #[arg(long,value_enum,default_value_t=Base64Format::Stand)]
    pub format: Base64Format,
}

impl CmdExecutor for Base64EncodeArgs {
    async fn execute(self) -> anyhow::Result<()> {
        let mut input = get_reader(&self.input)?;
        let encoded = base64_encode(&mut input, self.format)?;
        println!("{}", encoded);
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct Base64DecodeArgs {
    #[arg(long,value_parser=verify_file,help="Input file")]
    pub input: String,

    #[arg(long,value_enum,default_value_t=Base64Format::Stand)]
    pub format: Base64Format,
}

impl CmdExecutor for Base64DecodeArgs {
    async fn execute(self) -> anyhow::Result<()> {
        let mut input = get_reader(&self.input)?;
        let decoded = base64_decode(&mut input, self.format)?;
        println!("{}", decoded);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Base64Format {
    Stand,
    UrlSafe,
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
