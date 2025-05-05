pub mod convert_csv;
pub mod copy;
pub mod rand_pwd;

use std::path::Path;

use clap::{Parser, Subcommand};

pub use convert_csv::{CsvOptions, OutputFormat};
pub use copy::CopyOptions;
pub use rand_pwd::RandPwdOptions;

#[derive(Parser, Debug)]
#[command(version, about = "A treasure box for miscellaneous tools", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Subs,
}

#[derive(Subcommand, Debug)]
pub enum Subs {
    /// Copy a file from one location to another with mode if specified
    #[command(name = "copy")]
    Copy(CopyOptions),

    #[command(
        name = "csv",
        about = "Show csv file content or convert a csv file to other formats"
    )]
    Csv(CsvOptions),

    #[command(name = "randpwd", about = "Generate random password")]
    RandPwd(RandPwdOptions),
}

pub fn verify_file_exists(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}
