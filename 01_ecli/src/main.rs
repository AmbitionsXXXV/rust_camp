use std::{fs, io::Error, path::Path};

fn main() {
    let content = read_file("assets/juventus.csv");

    match content {
        Ok(c) => println!("Content: {}", c),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_file<P>(file_path: P) -> Result<String, Error>
where
    P: AsRef<Path>,
{
    fs::read_to_string(file_path)
}
