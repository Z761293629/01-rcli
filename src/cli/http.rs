use crate::{process::http::http_serve, CmdExecutor};

use super::verify_path;
use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum HttpSubCommands {
    Serve(HttpServeArgs),
}

impl CmdExecutor for HttpSubCommands {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommands::Serve(args) => args.execute().await,
        }
    }
}

#[derive(Debug, Args)]
pub struct HttpServeArgs {
    #[arg(short,long,value_parser=verify_path ,default_value = ".")]
    pub directory: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeArgs {
    async fn execute(self) -> anyhow::Result<()> {
        http_serve(self.directory, self.port).await?;
        Ok(())
    }
}
