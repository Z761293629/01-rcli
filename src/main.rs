use std::fs;

use base64::prelude::*;
use clap::Parser;
use rcli::cli::text::TextSubCommands;
use rcli::cli::{Base64SubCommands, Cli, SubCommands};
use rcli::process::text::{key_generate, sign_text, verify_text};
use rcli::process::{base64_decode, base64_encode, genpass, process_csv};
use rcli::utils::{get_content, get_reader};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        SubCommands::Csv(args) => {
            let output = if let Some(output) = args.output {
                output.clone()
            } else {
                format!("output.{}", args.format)
            };
            process_csv(&args.input, &output, args.format)?;
        }
        SubCommands::GenPass(args) => {
            genpass(
                args.len,
                args.no_upper,
                args.no_lower,
                args.no_number,
                args.no_symbol,
            )?;
        }
        SubCommands::Base64(cmd) => match cmd {
            Base64SubCommands::Base64Encode(args) => {
                let mut input = get_reader(&args.input)?;
                let encoded = base64_encode(&mut input, args.format)?;
                println!("{}", encoded);
            }
            Base64SubCommands::Base64Decode(args) => {
                let mut input = get_reader(&args.input)?;
                let decoded = base64_decode(&mut input, args.format)?;
                println!("{}", decoded);
            }
        },
        SubCommands::Text(cmd) => match cmd {
            TextSubCommands::TextSign(args) => {
                let mut input = get_reader(&args.input)?;
                let key = get_content(&args.key)?;
                let sig = sign_text(&mut input, &key, args.format)?;
                let encoded = BASE64_URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            TextSubCommands::TextVerify(args) => {
                let mut input = get_reader(&args.input)?;
                let sig = BASE64_URL_SAFE_NO_PAD.decode(&args.sig)?;
                let key = get_content(&args.key)?;
                let r = verify_text(&mut input, &key, &sig, args.format)?;
                println!("{}", r);
            }
            TextSubCommands::KeyGenerate(args) => {
                let keys = key_generate(args.format)?;
                for (k, v) in keys {
                    fs::write(args.output.join(k), v)?;
                }
            }
        },
    };
    Ok(())
}
