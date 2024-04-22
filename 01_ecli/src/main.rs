use anyhow::Result;
use serde::Deserialize;
use std::fs::File;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Player {
    // #[serde(rename = "Name")] 属性用于将 CSV 列名映射为结构字段名。
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    number: u8,
}

fn main() -> Result<()> {
    let file = File::open("assets/juventus.csv")?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for result in reader.deserialize() {
        let player: Player = result?;

        println!(
            "Name: {}, Position: {}, DOB: {}, Nationality: {}, Number: {}",
            player.name, player.position, player.dob, player.nationality, player.number,
        );
    }

    Ok(())
}
