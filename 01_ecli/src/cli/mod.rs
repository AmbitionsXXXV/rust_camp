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

    #[command(subcommand, about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
