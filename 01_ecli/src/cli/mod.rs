use anyhow::Result;
use clap::Parser;
use std::path::Path;

mod base64;
mod csv;
mod genpass;

pub use self::{base64::*, csv::*, genpass::*};

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

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(name = "base64", about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
}

fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}
