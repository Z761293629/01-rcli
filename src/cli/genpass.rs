use clap::Args;

#[derive(Debug, Args)]
pub struct GenPassArgs {
    #[arg(short, long, default_value_t = 8)]
    pub len: u8,

    #[arg(long, default_value_t = false)]
    pub no_upper: bool,

    #[arg(long, default_value_t = false)]
    pub no_lower: bool,

    #[arg(long, default_value_t = false)]
    pub no_number: bool,

    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}
