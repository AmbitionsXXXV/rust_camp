use anyhow::Result;
use csv::Reader;
use std::fs;

use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    // 因为下面 rdr.records() 是 mutable borrow, 所以这里需要 clone rdr.header()? 这个 mutable borrow
    let headers = rdr.headers()?.clone();

    for res in rdr.records() {
        let record = res?;

        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并为一个元组的迭代器 [(header, record), ..]
        // collect::<Value>() -> 将元组的迭代器转换为 JSON Value
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;

    Ok(())
}