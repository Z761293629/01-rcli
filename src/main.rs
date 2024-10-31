use clap::Parser;
use rcli::process_csv;
use rcli::{Cli, SubCommands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        SubCommands::Csv(args) => process_csv(&args.input, &args.output),
    }
}
