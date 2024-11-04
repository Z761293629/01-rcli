use clap::Parser;
use rcli::cli::{Base64SubCommands, Cli, SubCommands};
use rcli::process::{base64_decode, base64_encode, genpass, process_csv};

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
        SubCommands::GenPass(args) => {
            genpass(
                args.len,
                args.no_upper,
                args.no_lower,
                args.no_number,
                args.no_symbol,
            )?;
            Ok(())
        }
        SubCommands::Base64(sub) => match sub {
            Base64SubCommands::Base64Encode(args) => base64_encode(&args.input, args.format),
            Base64SubCommands::Base64Decode(args) => base64_decode(&args.input, args.format),
        },
    }
}
