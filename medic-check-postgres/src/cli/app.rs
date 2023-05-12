use clap::Args;
use clap::Parser;
use clap::Subcommand;
use std::fmt;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-check-postgres")]
/// Ensures that Postgres is running and configured for development.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Checks that postgres is running.
    Running(ConnectionArgs),
}

#[derive(Args, Debug)]
pub struct ConnectionArgs {
    #[arg(short, long, default_value = "1", value_hint = clap::ValueHint::CommandString)]
    pub connect_timeout: String,

    #[arg(short, long, env = "PGDATABASE", value_hint = clap::ValueHint::CommandString)]
    pub dbname: Option<String>,

    #[arg(long, env = "PGHOST", default_value = "localhost", value_hint = clap::ValueHint::CommandString)]
    pub host: String,

    #[arg(short = 'P', long, env = "PGPASSWORD", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub password: String,

    #[arg(short, long, env = "PGPORT", default_value = "5432", value_hint = clap::ValueHint::CommandString)]
    pub port: String,

    #[arg(short, long, default_value = "prefer", value_hint = clap::ValueHint::CommandString)]
    pub sslmode: String,

    #[arg(short, long, env = "PGUSER", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub user: String,
}

impl fmt::Display for ConnectionArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(dbname) = &self.dbname {
            write!(f, "dbname={} ", dbname)?;
        }
        write!(
            f,
            "host={} user={} port={} password={} sslmode={}",
            self.host, self.user, self.port, self.password, self.sslmode
        )
    }
}

impl CliArgs {
    pub fn new() -> Self {
        CliArgs::parse()
    }
}

impl Default for CliArgs {
    fn default() -> Self {
        Self::new()
    }
}
