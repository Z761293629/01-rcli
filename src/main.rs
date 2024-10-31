use clap::Parser;
use rcli::process_csv;
use rcli::{Cli, SubCommands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        SubCommands::Csv(args) => {
            let output = if let Some(output) = args.output {
                output.clone()
            } else {
                format!("output.{}", args.format)
            };
            process_csv(&args.input, &output, args.format)
        }
    }
}
