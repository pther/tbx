use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};
use std::{fs::File, io::Read};

use crate::cli::Base64Engine;

pub fn process_encode(input: &str, engine: Base64Engine) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match engine {
        Base64Engine::Standard => STANDARD.encode(&buf),
        Base64Engine::UrlSafe => URL_SAFE.encode(&buf),
    };
    println!("Encoded: {}", encode);
    Ok(())
}

pub fn process_decode(input: &str, engine: Base64Engine) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let decode = match engine {
        Base64Engine::Standard => STANDARD.decode(&buf)?,
        Base64Engine::UrlSafe => URL_SAFE.decode(&buf)?,
    };

    let decode = String::from_utf8(decode)?;
    println!("Decoded: {}", decode);
    Ok(())
}
