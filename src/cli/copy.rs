use clap::{arg, Parser};

use super::verify_file_exists;

#[derive(Parser, Debug)]
pub struct CopyOptions {
    #[arg(short, long, value_name = "source file", aliases = ["src", "srcfile", "src_file", "sourcefile"], required = true, help = "Source file to copy from", value_parser = verify_file_exists)]
    pub source: String,

    #[arg(
        short,
        long,
        value_name = "target file",
        required = true,
        help = "Target file to copy to"
    )]
    pub target: String,

    #[arg(
        short,
        long,
        value_name = "mode",
        help = "The mode of how file is copied",
        value_parser = ["overwrite", "incr"],
        default_value = "overwrite"
    )]
    pub mode: String,

    #[arg(
        short,
        long,
        value_name = "replica count",
        help = "How many times to copy the file",
        required_if_eq("mode", "incr"),
        default_value_t = 1
    )]
    pub replica: u8,

    #[arg(
        short,
        long,
        value_name = "starting number",
        help = "Starting number appending to the end of file name",
        required_if_eq("mode", "incr"),
        default_value_t = 1
    )]
    pub from: u8,
}
