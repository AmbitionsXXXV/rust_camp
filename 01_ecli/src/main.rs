use std::{fs, path::Path};

use anyhow::Result;

// 当不需要传递错误信息时，可以使用 `Result<(), Error>` 作为返回值
fn main() -> Result<()> {
    let content = read_file("assets/juventus.csv")?;

    println!("{}", content);
    Ok(())
}

// 适合在需要传递错误信息时使用 `Result<String, Error>` 作为返回值
// fn main() {
//     let content = read_file("assets/juventus.csv");
//
//     match content {
//         Ok(c) => println!("Content: {}", c),
//         Err(e) => println!("Error: {}", e),
//     }
// }

fn read_file<P>(file_path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(file_path)?; // err convert to Result

    Ok(content)
}
