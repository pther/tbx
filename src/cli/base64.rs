use super::verify_file_exists;
use clap::Parser;
use core::fmt;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub enum Base64Mode {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOptions),

    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOptions),
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOptions {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_engine, default_value = "Standard")]
    pub engine: Base64Engine,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOptions {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_engine, default_value = "Standard")]
    pub engine: Base64Engine,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Engine {
    Standard,
    UrlSafe,
}

fn parse_base64_engine(engine: &str) -> Result<Base64Engine, anyhow::Error> {
    engine.parse()
}

impl FromStr for Base64Engine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Engine::Standard),
            "urlsafe" => Ok(Base64Engine::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 engine")),
        }
    }
}

impl fmt::Display for Base64Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base64Engine::Standard => write!(f, "Standard"),
            Base64Engine::UrlSafe => write!(f, "UrlSafe"),
        }
    }
}

impl From<Base64Engine> for &'static str {
    fn from(engine: Base64Engine) -> Self {
        match engine {
            Base64Engine::Standard => "Standard",
            Base64Engine::UrlSafe => "UrlSafe",
        }
    }
}
