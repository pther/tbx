use clap::Parser;

#[derive(Parser, Debug)]
pub enum Base64Mode {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOptions),

    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOptions),
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOptions {
    /// The string to encode
    pub input: String,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOptions {
    /// The base64 string to decode
    pub input: String,
}
