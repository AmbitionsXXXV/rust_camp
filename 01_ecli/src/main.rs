use anyhow::{Ok, Result};
use clap::Parser;
use ecli::{
    get_reader, process_csv, process_decode, process_encode, process_gen_pass, Base64SubCommand,
    Opts, SubCommand,
};
use zxcvbn::zxcvbn;

fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = &opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            let pass = process_gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;

            // 验证密码强度
            let estimate = zxcvbn(&pass, &[])?;
            eprintln!("Password strength: {}", estimate.score());

            println!("{}", pass);
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let mut reader = crate::get_reader(&opts.input)?;
                process_encode(&mut reader, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                let mut reader = crate::get_reader(&opts.input)?;
                process_decode(&mut reader, opts.format)?;
            }
        },
    }

    Ok(())
}
