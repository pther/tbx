use clap::{arg, command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "A treasure box for miscellaneous tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Subs>,
}

#[derive(Subcommand, Debug)]
enum Subs {
    /// Copy a file from one location to another with mode if specified
    Copy {
        #[arg(short, long, value_name = "source file", aliases = ["src", "srcfile", "src_file", "sourcefile"], required = true, help = "Source file to copy from")]
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
        mode: Option<String>,

        #[arg(
            short,
            long,
            value_name = "replica count",
            help = "How many times to copy the file"
        )]
        replica: Option<u8>,

        #[arg(
            short,
            long,
            value_name = "starting number",
            help = "Starting number appending to the end of file name"
        )]
        from: Option<u8>,

        #[arg(
            short,
            long,
            value_name = "suffix length",
            help = "Length of suffix to be appended to the file name",
            default_value = "1"
        )]
        length: u8,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Subs::Copy {
            source,
            target,
            mode,
            from,
            length,
            replica,
        }) => {
            println!("Copying from {} to {} with mode {:?}, starting number {:?}, suffix length {}, and replica count {:?}", source, target, mode, from, length, replica);
            // Call the copy function here
        }
        None => {
            println!("No command was provided. Use --help for more information.");
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_main() {
        assert_eq!(1 + 1, 2);
    }
}
