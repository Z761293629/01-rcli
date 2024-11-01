use anyhow::Result;
use rand::prelude::*;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"~!@#$%^&*()_-+=";

pub fn genpass(
    len: u8,
    no_upper: bool,
    no_lower: bool,
    no_number: bool,
    no_symbol: bool,
) -> Result<()> {
    let mut chars = Vec::new();
    let mut password = Vec::new();
    let mut rng = rand::thread_rng();
    if !no_upper {
        chars.extend_from_slice(UPPER);
        let c = *UPPER.choose(&mut rng).expect("char must not empty");
        password.push(c);
    }

    if !no_lower {
        chars.extend_from_slice(LOWER);
        let c = *LOWER.choose(&mut rng).expect("char must not empty");
        password.push(c);
    }

    if !no_number {
        chars.extend_from_slice(NUMBER);
        let c = *NUMBER.choose(&mut rng).expect("char must not empty");
        password.push(c);
    }

    if !no_symbol {
        chars.extend_from_slice(SYMBOL);
        let c = *SYMBOL.choose(&mut rng).expect("char must not empty");
        password.push(c);
    }

    for _ in 0..(len - password.len() as u8) {
        let c = *chars.choose(&mut rng).expect("char must not empty");
        password.push(c);
    }
    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;
    println!("{}", &password);
    eprintln!("score : {}", zxcvbn::zxcvbn(&password, &[]).score());

    Ok(())
}
