use crate::cli::base64::Base64Format;
use std::{
    fs::File,
    io::{self, Read},
};

use anyhow::Ok;
use base64::prelude::*;

pub fn base64_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = reader(input)?;

    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;

    let encode = match format {
        Base64Format::Stand => BASE64_STANDARD.encode(&content),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(&content),
    };
    println!("{}", encode);
    Ok(())
}

pub fn base64_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = reader(input)?;
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    let content = content.trim();
    let decode = match format {
        Base64Format::Stand => BASE64_STANDARD.decode(content)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(content)?,
    };
    println!("{}", String::from_utf8(decode)?);
    Ok(())
}

fn reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_base64_encode() {
        assert!(base64_encode("Cargo.toml", Base64Format::Stand).is_ok());
    }

    #[test]
    fn test_base64_decode() {
        assert!(base64_decode("fixtures/passwordb64.txt", Base64Format::Stand).is_ok());
    }
}
