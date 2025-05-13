mod cli;
mod process;

//re-export the cli, copy, convert_csv and rand_pwd module for easier access
// This allows users to use `tbx::Cli` and `tbx::copy_file` directly
pub use cli::{Base64Mode, Cli, CopyOptions, CsvOptions, OutputFormat, RandPwdOptions, Subs};
pub use process::{
    copy_file, generate_random_password, process_csv, process_decode, process_encode,
};
