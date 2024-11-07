use enum_dispatch::enum_dispatch;

pub mod cli;
pub mod process;
pub mod utils;

use crate::cli::{
    base64::{Base64DecodeArgs, Base64EncodeArgs, Base64SubCommands},
    csv::CsvArgs,
    genpass::GenPassArgs,
    http::{HttpServeArgs, HttpSubCommands},
    text::{
        KeyGenerateArgs, TextDecryptArgs, TextEncryptArgs, TextSignArgs, TextSubCommands,
        TextVerifyArgs,
    },
    SubCommands,
};

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
