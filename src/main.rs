use clap::Parser;
use tbx::{copy_file, generate_random_password, process_csv, Cli, Subs};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Subs::Copy(options) => {
            copy_file(
                &options.source,
                &options.target,
                &options.mode,
                options.from,
                options.replica,
            )?;
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
        Subs::RandPwd(options) => {
            let password = generate_random_password(
                options.length,
                options.uppercase,
                options.lowercase,
                options.number,
                options.symbol,
            )?;
            println!("Generated password: {}", password);
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
