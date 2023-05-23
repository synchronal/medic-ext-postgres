use clap::Args;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use std::fmt;

#[derive(Clone, Debug, Parser, ValueEnum)]
pub enum SslMode {
    #[clap(rename_all = "kebab-case")]
    Prefer,
}

impl std::fmt::Display for SslMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SslMode::Prefer => write!(f, "prefer"),
        }
    }
}

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
    /// connection timeout in seconds
    #[arg(short, long, default_value = "5", value_hint = clap::ValueHint::CommandString)]
    pub connect_timeout: String,

    #[arg(short, long, env = "PGDATABASE", value_hint = clap::ValueHint::CommandString)]
    pub dbname: Option<String>,

    #[arg(long, env = "PGHOST", default_value = "localhost", value_hint = clap::ValueHint::CommandString)]
    pub host: String,

    #[arg(short = 'P', long, env = "PGPASSWORD", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub password: String,

    #[arg(short, long, env = "PGPORT", default_value = "5432", value_hint = clap::ValueHint::CommandString)]
    pub port: String,

    #[arg(long, default_value_t = SslMode::Prefer)]
    pub sslmode: SslMode,

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
            "host={} user={} port={} password={} sslmode={} connect_timeout={}",
            self.host, self.user, self.port, self.password, self.sslmode, self.connect_timeout
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
