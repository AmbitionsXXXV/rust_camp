use anyhow::Result;
use csv::Reader;
use std::{collections::HashMap, fs};
use uuid::Uuid;

use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut rdr = Reader::from_path(input)?;
    // 因为下面 rdr.records() 是 mutable borrow, 所以这里需要 clone rdr.header()? 这个 mutable borrow
    let headers = rdr.headers()?.clone();
    let mut values = HashMap::new();

    for res in rdr.records() {
        let record = res?;

        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并为一个元组的迭代器 [(header, record), ..]
        // collect::<Value>() -> 将元组的迭代器转换为 Value
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
