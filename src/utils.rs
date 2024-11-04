use std::{
    fs::File,
    io::{self, Read},
};

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn get_content(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;

    let mut content = Vec::new();
    reader.read_to_end(&mut content)?;
    Ok(content)
}
