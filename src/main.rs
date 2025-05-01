#![allow(unused)]

use clap::{arg, command, Parser, Subcommand};
use tbx::{copy_file, process_csv, Cli, OutputFormat, Subs};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Subs::Copy {
            source,
            target,
            mode,
            from,
            replica,
        } => {
            copy_file(&source, &target, &mode, from, replica)?;
        }
        Subs::Csv(options) => {
            let output = if let Some(output) = options.output {
                output
            } else {
                // If no output is provided, use the input file name with the new format
                format!("output.{}", options.format)
            };
            process_csv(&options.input, &output, options.format)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_main() {
        assert_eq!(1 + 1, 2);
    }
}
