use clap::Parser;
use rcli::{genpass, process_csv};
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
        SubCommands::GenPass(args) => genpass(
            args.len,
            args.no_upper,
            args.no_lower,
            args.no_number,
            args.no_symbol,
        ),
    }
}
