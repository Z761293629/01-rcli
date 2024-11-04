use super::{verify_file, verify_path};
use clap::{Args, Subcommand};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Subcommand)]
pub enum TextSubCommands {
    #[command(
        name = "sign",
        about = "sign a text with a private/session key and return a signature"
    )]
    TextSign(TextSignArgs),

    #[command(name = "verify")]
    TextVerify(TextVerifyArgs),

    #[command(name = "generate")]
    KeyGenerate(KeyGenerateArgs),
}

#[derive(Debug, Args)]
pub struct KeyGenerateArgs {
    #[arg(long, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(long,value_parser=verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Args)]
pub struct TextVerifyArgs {
    #[arg(long,value_parser=verify_file,default_value="-")]
    pub input: String,

    #[arg(long,value_parser=verify_file)]
    pub key: String,

    #[arg(long)]
    pub sig: String,

    #[arg(long, value_parser=parse_text_sign_format,default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Args)]
pub struct TextSignArgs {
    #[arg(long,value_parser=verify_file,default_value="-")]
    pub input: String,

    #[arg(long,value_parser=verify_file)]
    pub key: String,

    #[arg(long, value_parser=parse_text_sign_format,default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("invalid format : {}", s)),
        }
    }
}
