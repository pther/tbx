use std::path::Path;

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
            default_value = "overwrite"
        )]
        mode: String,

        #[arg(
            short,
            long,
            value_name = "replica count",
            help = "How many times to copy the file"
        )]
        replica: u8,

        #[arg(
            short,
            long,
            value_name = "starting number",
            help = "Starting number appending to the end of file name"
        )]
        from: u8,

        #[arg(
            short,
            long,
            value_name = "suffix length",
            help = "Length of suffix to be appended to the file name",
            default_value = "1"
        )]
        length: u8,
    },

    #[command(name = "csv", about = "Show csv or convert a file to other formats")]
    Csv(CsvOptions),
}

#[derive(Parser, Debug)]
pub struct CsvOptions {
    #[arg(short, long, value_name = "input", help = "Input file to be converted", value_parser = verify_file_exists)]
    pub input: String,

    #[arg(
        short,
        long,
        value_name = "output",
        help = "Output file to convert to",
        default_value = "output.json"
    )]
    pub output: String,

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

pub fn verify_file_exists(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}
