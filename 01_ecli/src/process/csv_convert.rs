use anyhow::Result;
use csv::Reader;
use std::{collections::HashMap, fs};
use uuid::Uuid;

use crate::cli::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let headers = rdr.headers()?.clone();
    let mut values = HashMap::new();

    for res in rdr.records() {
        let record = res?;

        let value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        values.insert(format!("data_{}", Uuid::new_v4()), value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&values)?,
        OutputFormat::Yaml => serde_yaml::to_string(&values)?,
        OutputFormat::Toml => toml::to_string(&values)?,
    };

    fs::write(output, content)?;

    Ok(())
}
