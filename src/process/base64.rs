use crate::cli::base64::Base64Format;
use std::io::Read;

use anyhow::Ok;
use base64::prelude::*;

pub fn base64_encode(input: &mut dyn Read, format: Base64Format) -> anyhow::Result<String> {
    let mut content = Vec::new();
    input.read_to_end(&mut content)?;

    let encoded = match format {
        Base64Format::Stand => BASE64_STANDARD.encode(&content),
        Base64Format::UrlSafe => BASE64_URL_SAFE.encode(&content),
    };
    Ok(encoded)
}

pub fn base64_decode(input: &mut dyn Read, format: Base64Format) -> anyhow::Result<String> {
    let mut content = String::new();
    input.read_to_string(&mut content)?;
    let content = content.trim();
    let decoded = match format {
        Base64Format::Stand => BASE64_STANDARD.decode(content)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(content)?,
    };

    Ok(String::from_utf8(decoded)?)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::get_reader;

    #[test]
    fn test_base64_encode() -> anyhow::Result<()> {
        let file = "Cargo.toml";
        let mut input = get_reader(file)?;
        assert!(base64_encode(&mut input, Base64Format::Stand).is_ok());
        Ok(())
    }

    #[test]
    fn test_base64_decode() -> anyhow::Result<()> {
        let file = "fixtures/passwordb64.txt";
        let mut input = get_reader(file)?;
        assert!(base64_decode(&mut input, Base64Format::Stand).is_ok());
        Ok(())
    }
}
