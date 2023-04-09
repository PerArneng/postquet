
use clap::{Parser};
use std::fmt::{Display, Formatter, Result};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {

    #[clap(short = 'n', long, value_name = "HOSTNAME")]
    pub hostname: String,

    #[clap(short, long, value_name = "USERNAME")]
    pub username: String,

    #[clap(short = 'w', long, value_name = "PASSWORD")]
    pub password: String,

    #[clap(short = 'p', long, value_name = "PORT", default_value = "5432")]
    pub port: u16,

    #[clap(short, long, value_name = "DATABASE")]
    pub database: String,

    #[clap(short, long, value_name = "TABLE")]
    pub table: String,

}

impl Display for CliArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "CliArgs {{ hostname: {}, username: {}, password: ****, port: {}, database: {}, table: {} }}",
            self.hostname, self.username, self.port, self.database, self.table
        )
    }
}

pub fn parse_cli_args() -> CliArgs {
    CliArgs::parse()
}
