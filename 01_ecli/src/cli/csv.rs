use std::{fmt, str::FromStr};

use crate::OutputFormat;

use super::verify_input_file;
use clap::{ArgAction, Parser};

/// CSV options
#[derive(Debug, Parser)]
#[clap(name = "CSV Options", author, version, about = "Convert CSV to JSON")]
pub struct CsvOpts {
    /// Input CSV file, value_parser 参数指定验证函数
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    /// Output JSON file, optionalq
    #[arg(short, long)]
    pub output: Option<String>,

    /// Output format, default is json, supported formats: json, yaml, toml
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    /// Delimiter, default is comma
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// CSV has header
    #[arg(short, long, action = ArgAction::Set, default_value_t = true)]
    pub csv_header: bool,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => anyhow::bail!("Invalid format: {}", s),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
