use super::verify_path;
use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum HttpSubCommands {
    Serve(HttpServeArgs),
}

#[derive(Debug, Args)]
pub struct HttpServeArgs {
    #[arg(short,long,value_parser=verify_path ,default_value = ".")]
    pub directory: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
