use clap::Parser;

#[derive(Parser, Debug)]
pub struct RandPwdOptions {
    #[arg(
        short,
        long,
        value_name = "length",
        help = "Length of the password",
        default_value_t = 16
    )]
    pub length: u8,

    #[arg(
        long,
        help = "Does password contain uppercase letter",
        default_value_t = true
    )]
    pub uppercase: bool,

    #[arg(
        long,
        help = "Does password contain lowercase letter",
        default_value_t = true
    )]
    pub lowercase: bool,

    #[arg(long, help = "Does password contain number", default_value_t = true)]
    pub number: bool,

    #[arg(long, help = "Does password contain symbol", default_value_t = true)]
    pub symbol: bool,
}
