use anyhow::Result;
use clap::{ArgAction, Parser};
use std::{fmt, path::Path, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => anyhow::bail!("Invalid format: {}", s),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Debug, Parser)]
#[command(name="ecli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

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

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    /// Delimiter, default is comma
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// CSV has header
    #[arg(short, long, action = ArgAction::Set, default_value_t = true)]
    pub csv_header: bool,
}

fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}
