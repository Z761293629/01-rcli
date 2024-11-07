use crate::{
    process::text::{decrypt_text, encrypt_text, key_generate, sign_text, verify_text},
    utils::{get_content, get_reader},
    CmdExecutor,
};

use super::{verify_file, verify_path};
use base64::prelude::*;
use clap::{Args, Subcommand, ValueEnum};
use enum_dispatch::enum_dispatch;
use std::{io::Cursor, path::PathBuf, str::FromStr};
use tokio::fs;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
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

    #[command(name = "encrypt")]
    TextEncrypt(TextEncryptArgs),

    #[command(name = "decrypt")]
    TextDecrypt(TextDecryptArgs),
}

#[derive(Debug, Args)]
pub struct TextDecryptArgs {
    #[arg(long)]
    pub key: String,

    #[arg(long,value_parser=verify_file,default_value="-")]
    pub input: String,
}

impl CmdExecutor for TextDecryptArgs {
    async fn execute(self) -> anyhow::Result<()> {
        let content = get_content(&self.input)?;
        let content = BASE64_URL_SAFE_NO_PAD.decode(content)?;
        let plain = decrypt_text(&mut Cursor::new(&content), &self.key)?;
        println!("{}", String::from_utf8(plain)?);
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct TextEncryptArgs {
    #[arg(long)]
    pub key: String,

    #[arg(long,value_parser=verify_file,default_value="-")]
    pub input: String,
}

impl CmdExecutor for TextEncryptArgs {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let encrypt = encrypt_text(&mut reader, &self.key)?;
        let encrtpt = BASE64_URL_SAFE_NO_PAD.encode(encrypt);
        println!("{}", encrtpt);
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct KeyGenerateArgs {
    #[arg(long, value_enum,default_value_t=TextSignFormat::Blake3)]
    pub format: TextSignFormat,

    #[arg(long,value_parser=verify_path)]
    pub output: PathBuf,
}

impl CmdExecutor for KeyGenerateArgs {
    async fn execute(self) -> anyhow::Result<()> {
        let keys = key_generate(self.format)?;
        for (k, v) in keys {
            fs::write(self.output.join(k), v).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct TextVerifyArgs {
    #[arg(long,value_parser=verify_file,default_value="-")]
    pub input: String,

    #[arg(long,value_parser=verify_file)]
    pub key: String,

    #[arg(long)]
    pub sig: String,

    #[arg(long, value_enum,default_value_t=TextSignFormat::Blake3)]
    pub format: TextSignFormat,
}

impl CmdExecutor for TextVerifyArgs {
    async fn execute(self) -> anyhow::Result<()> {
        let mut input = get_reader(&self.input)?;
        let sig = BASE64_URL_SAFE_NO_PAD.decode(&self.sig)?;
        let key = get_content(&self.key)?;
        let r = verify_text(&mut input, &key, &sig, self.format)?;
        println!("{}", r);
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct TextSignArgs {
    #[arg(long,value_parser=verify_file,default_value="-")]
    pub input: String,

    #[arg(long,value_parser=verify_file)]
    pub key: String,

    #[arg(long, value_enum,default_value_t=TextSignFormat::Blake3)]
    pub format: TextSignFormat,
}
impl CmdExecutor for TextSignArgs {
    async fn execute(self) -> anyhow::Result<()> {
        let mut input = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = sign_text(&mut input, &key, self.format)?;
        let encoded = BASE64_URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
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
