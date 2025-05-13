mod base64;
mod convert_csv;
mod copy;
mod rand_pwd;

use clap::{Parser, Subcommand};
use std::path::Path;

pub use base64::{Base64Engine, Base64Mode};
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

    #[command(subcommand)]
    Base64(Base64Mode),
}

pub fn verify_file_exists(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file_exists() {
        assert_eq!(verify_file_exists("-"), Ok("-".into()));
        assert_eq!(verify_file_exists("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file_exists("non-exist"), Err("File does not exist"));
    }
}
