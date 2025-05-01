mod cli;
mod process;

//re-export the cli and process modules for easier access
// This allows users to use `tbx::Cli` and `tbx::process_csv` directly
pub use cli::{Cli, OutputFormat, Subs};
pub use process::{copy_file, process_csv};
