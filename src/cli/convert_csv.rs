use std::{fmt, str::FromStr};

use clap::Parser;

use super::verify_file_exists;

#[derive(Parser, Debug)]
pub struct CsvOptions {
    #[arg(short, long, value_name = "input", help = "Input file to be read or converted", value_parser = verify_file_exists)]
    pub input: String,

    #[arg(short, long, value_name = "output", help = "Output file to convert to")]
    pub output: Option<String>,

    #[arg(
        short,
        long,
        value_name = "format",
        help = "Output format",
        value_parser = parse_format,
        default_value = "json"
    )]
    pub format: OutputFormat,

    #[arg(
        short,
        long,
        value_name = "delimiter",
        help = "Delimiter used in the file",
        default_value_t = ','
    )]
    pub delimiter: char,

    #[arg(
        long,
        value_name = "header",
        help = "Has header in the file",
        default_value_t = true
    )]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Unsupported output format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

pub fn parse_format(s: &str) -> Result<OutputFormat, anyhow::Error> {
    s.parse()
}
