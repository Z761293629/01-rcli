use chacha20poly1305::{
    aead::{self, Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use ed25519_dalek::Signature;
use ed25519_dalek::SigningKey;
use ed25519_dalek::{ed25519::signature::Signer, VerifyingKey};
use rand::rngs::OsRng;
use std::{collections::HashMap, io::Read};

use crate::cli::text::TextSignFormat;
use crate::process::genpass::genpass;

pub trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerifyer {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    signing_key: SigningKey,
}

struct Ed25519Verifyer {
    verifying_key: VerifyingKey,
}

impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key: [u8; 32] = key.try_into()?;
        Ok(Self { key })
    }

    pub fn generate() -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let key = genpass(32, false, false, false, false)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerifyer for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        Ok(self.sign(reader)? == sig)
    }
}

impl Ed25519Signer {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = key.try_into()?;
        let signing_key = SigningKey::from_bytes(key);
        Ok(Self { signing_key })
    }

    pub fn generate() -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let mut map = HashMap::new();
        map.insert("ed25519.sk", signing_key.as_bytes().to_vec());
        let verifying_key: VerifyingKey = signing_key.verifying_key();
        map.insert("ed25519.pk", verifying_key.as_bytes().to_vec());
        Ok(map)
    }
}

impl Ed25519Verifyer {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref().try_into()?;
        let verifying_key = VerifyingKey::from_bytes(key)?;
        Ok(Self { verifying_key })
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.signing_key.sign(&buf);
        Ok(sig.to_vec())
    }
}

impl TextVerifyer for Ed25519Verifyer {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut content = Vec::new();
        reader.read_to_end(&mut content)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.verifying_key.verify_strict(&content, &sig).is_ok())
    }
}

pub fn sign_text(
    input: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat,
) -> anyhow::Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };
    signer.sign(input)
}

pub fn verify_text(
    input: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    let verifyer: Box<dyn TextVerifyer> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifyer::try_new(key)?),
    };
    verifyer.verify(input, sig)
}

pub fn key_generate(format: TextSignFormat) -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

pub fn encrypt_text(reader: &mut dyn Read, key: impl AsRef<[u8]>) -> anyhow::Result<Vec<u8>> {
    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;
    let key = Key::from_slice(key.as_ref());
    let cipher = ChaCha20Poly1305::new(key);
    // 96-bits; unique per message
    let nonce = ChaCha20Poly1305::generate_nonce(&mut aead::OsRng);
    let ciphertext = cipher.encrypt(&nonce, content.as_ref());

    match ciphertext {
        Ok(mut v) => {
            let mut nonce = nonce.to_vec();
            nonce.append(&mut v);
            Ok(nonce)
        }
        Err(e) => Err(anyhow::anyhow!(format!("encrypt error {:?}", e))),
    }
}

const NONCE_LEN: usize = 12;

pub fn decrypt_text(reader: &mut dyn Read, key: impl AsRef<[u8]>) -> anyhow::Result<Vec<u8>> {
    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;
    let key = Key::from_slice(key.as_ref());
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(&content[0..NONCE_LEN]);
    let plaint = cipher.decrypt(nonce, &content[NONCE_LEN..]);
    match plaint {
        Ok(plaint) => Ok(plaint),
        Err(e) => Err(anyhow::anyhow!(format!("decrypt error {:?}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const KEY: &[u8] = include_bytes!("../../fixtures/blake3.key");

    #[test]
    fn test_blake3_sign() -> anyhow::Result<()> {
        let blake = Blake3::try_new(KEY)?;
        let mut data = "hello!".as_bytes();
        let _ = blake.sign(&mut data)?;
        Ok(())
    }

    #[test]
    fn test_blake3_verify() -> anyhow::Result<()> {
        let blake = Blake3::try_new(KEY)?;
        let sig = blake.sign(&mut "hello!".as_bytes())?;
        assert!(blake.verify(&mut "hello!".as_bytes(), &sig)?);
        Ok(())
    }

    #[test]
    fn test_ed25519() -> anyhow::Result<()> {
        let singing_key = include_bytes!("../../fixtures/ed25519.sk");

        let signer = Ed25519Signer::try_new(singing_key)?;

        let sig = signer.sign(&mut "hello".as_bytes())?;
        let verifying_key = include_bytes!("../../fixtures/ed25519.pk");

        let verifyer = Ed25519Verifyer::try_new(verifying_key)?;
        let r = verifyer.verify(&mut "hello".as_bytes(), &sig)?;
        assert!(r);
        Ok(())
    }

    #[test]
    fn test_chacha20() -> anyhow::Result<()> {
        let key = b"yXHLHs9WcdpkTV8elon1XgoGtdy5anJR";
        let encrypt = encrypt_text(&mut "hello world".as_bytes(), key)?;
        let mut cursor = std::io::Cursor::new(&encrypt);
        let r = decrypt_text(&mut cursor, key)?;
        assert_eq!(String::from_utf8(r)?, "hello world");
        Ok(())
    }
}
