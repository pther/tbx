use core::fmt;
use std::{path::Path, str::FromStr};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "A treasure box for miscellaneous tools", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Subs,
}

#[derive(Subcommand, Debug)]
pub enum Subs {
    /// Copy a file from one location to another with mode if specified
    Copy {
        #[arg(short, long, value_name = "source file", aliases = ["src", "srcfile", "src_file", "sourcefile"], required = true, help = "Source file to copy from", value_parser = verify_file_exists)]
        source: String,

        #[arg(
            short,
            long,
            value_name = "target file",
            required = true,
            help = "Target file to copy to"
        )]
        target: String,

        #[arg(
            short,
            long,
            value_name = "mode",
            help = "The mode of how file is copied",
            value_parser = ["overwrite", "incr"],
            default_value = "overwrite"
        )]
        mode: String,

        #[arg(
            short,
            long,
            value_name = "replica count",
            help = "How many times to copy the file",
            required_if_eq("mode", "incr"),
            default_value_t = 1
        )]
        replica: u8,

        #[arg(
            short,
            long,
            value_name = "starting number",
            help = "Starting number appending to the end of file name",
            required_if_eq("mode", "incr"),
            default_value_t = 1
        )]
        from: u8,
    },

    #[command(
        name = "csv",
        about = "Show csv file content or convert a csv file to other formats"
    )]
    Csv(CsvOptions),

    #[command(name = "randpwd", about = "Generate random password")]
    RandPwd(RandPwdOptions),
}

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

#[derive(Parser, Debug)]
pub struct RandPwdOptions {
    #[arg(
        short,
        long,
        value_name = "length",
        help = "Length of the password",
        default_value_t = 16
    )]
    pub length: u8,

    #[arg(
        long,
        help = "Does password contain uppercase letter",
        default_value_t = true
    )]
    pub uppercase: bool,

    #[arg(
        long,
        help = "Does password contain lowercase letter",
        default_value_t = true
    )]
    pub lowercase: bool,

    #[arg(long, help = "Does password contain number", default_value_t = true)]
    pub number: bool,

    #[arg(long, help = "Does password contain symbol", default_value_t = true)]
    pub symbol: bool,
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

pub fn verify_file_exists(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

pub fn parse_format(s: &str) -> Result<OutputFormat, anyhow::Error> {
    s.parse()
}
