use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::io::Read;

use crate::Base64Format;

pub enum DecodedData {
    /// 当解码后的数据是字符串时，使用 Text 变体
    Text(String),
    /// 当解码后的数据是字节数据时，使用 Binary 变体
    Binary(Vec<u8>),
}

impl std::fmt::Debug for DecodedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodedData::Text(s) => write!(f, "Text({})", s),
            DecodedData::Binary(b) => write!(f, "Binary({:?})", b),
        }
    }
}

pub fn process_encode(reader: &mut dyn Read, format: Base64Format) -> Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    Ok(encoded)
}

pub fn process_decode(reader: &mut dyn Read, format: Base64Format) -> Result<DecodedData> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    println!("buffer {:?}", buf.len());
    // 移除多余的换行符
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    // 尝试将解码后的数据转换为 UTF-8 字符串，如果失败则返回字节数据
    match String::from_utf8(decoded.clone()) {
        Ok(s) => Ok(DecodedData::Text(s)),
        Err(_) => Ok(DecodedData::Binary(decoded)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_reader;

    #[test]
    fn test_process_encode() -> Result<()> {
        let input = "Cargo.toml";
        let mut reader = get_reader(input)?;
        let format = Base64Format::Standard;
        let encoded = process_encode(&mut reader, format)?;
        assert!(!encoded.is_empty());

        Ok(())
    }

    // #[test]
    // fn test_process_decode() -> Result<()> {
    //     let input = "fixtures/b64.txt";
    //     let mut reader = get_reader(input)?;
    //     let format = Base64Format::UrlSafe;
    //     process_decode(&mut reader, format)?;

    //     Ok(())
    // }
}
