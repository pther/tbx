mod cli;
mod process;

//re-export the cli, copy, convert_csv and rand_pwd module for easier access
// This allows users to use `tbx::Cli` and `tbx::copy_file` directly
pub use cli::{Cli, OutputFormat, Subs};
pub use process::copy_file;
pub use process::generate_random_password;
pub use process::process_csv;
