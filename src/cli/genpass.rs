use crate::process::genpass;
use crate::CmdExecutor;
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
impl CmdExecutor for GenPassArgs {
    async fn execute(self) -> anyhow::Result<()> {
        let password = genpass(
            self.len,
            self.no_upper,
            self.no_lower,
            self.no_number,
            self.no_symbol,
        )?;
        println!("{}", password);
        eprintln!("score : {}", zxcvbn::zxcvbn(&password, &[]).score());
        Ok(())
    }
}
