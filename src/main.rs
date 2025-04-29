#![allow(unused)]

use clap::{arg, command, Parser, Subcommand};
use tbx::{copy_file, process_csv, Cli, Subs};

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
            process_csv(&options.input, &options.output)?;
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
