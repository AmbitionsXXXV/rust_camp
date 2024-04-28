use anyhow::Result;
use clap::{ArgAction, Parser};
use std::path::Path;

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
    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    // #[arg(short, long)]
    // pub format: OutputFormat,
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
