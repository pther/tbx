use clap::Parser;
use super::verify_file_exists;

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
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOptions {
    #[arg(short, long, value_parser = verify_file_exists, default_value = "-")]
    pub input: String,
}
