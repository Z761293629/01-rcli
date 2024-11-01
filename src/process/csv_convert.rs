use serde::{Deserialize, Serialize};
use std::fs;

use crate::cli::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut result = Vec::with_capacity(128);
    let header = reader.headers()?.clone();
    for record in reader.records() {
        let record = record?;
        let json_value = header
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        result.push(json_value);
    }

    let string = match format {
        OutputFormat::JSON => serde_json::to_string_pretty(&result)?,
        OutputFormat::YAML => serde_yaml::to_string(&result)?,
    };

    fs::write(output, string)?;
    Ok(())
}
