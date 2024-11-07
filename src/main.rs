use clap::Parser;
use rcli::cli::Cli;
use rcli::CmdExecutor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    cli.command.execute().await?;
    Ok(())
}
