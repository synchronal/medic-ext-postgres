use clap::Args;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use std::fmt;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "medic-check-postgres")]
/// Ensures that postgres is running and configured for development.
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Checks that postgres is running from a specific data directory.
    DataDirectory(DataDirectoryCheckArgs),
    /// Checks that a specific database exists.
    DatabaseExists(DatabaseCheckArgs),
    /// Checks that a specific database exists.
    RoleExists(RoleCheckArgs),
    /// Checks that postgres is running.
    Running(DefaultConnectionArgs),
}

#[derive(Args, Debug)]
pub struct DefaultConnectionArgs {
    /// connection timeout in seconds
    #[arg(short, long, default_value = "5", value_hint = clap::ValueHint::CommandString)]
    pub connect_timeout: String,

    /// optional database used to connect to postgres
    #[arg(short, long, default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub dbname: String,

    #[arg(long, env = "PGHOST", default_value = "localhost", value_hint = clap::ValueHint::CommandString)]
    pub host: String,

    #[arg(short = 'P', long, env = "PGPASSWORD", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub password: String,

    #[arg(short, long, env = "PGPORT", default_value = "5432", value_hint = clap::ValueHint::CommandString)]
    pub port: String,

    #[arg(long, default_value_t = SslMode::Prefer)]
    pub sslmode: SslMode,

    /// user for connecting to postgres.
    #[arg(short, long, env = "PGUSER", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub user: String,

    /// optional remedy if command fails
    #[arg(long, value_hint = clap::ValueHint::CommandString)]
    pub remedy: Option<String>,
}

impl fmt::Display for DefaultConnectionArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dbname={} host={} user={} port={} password={} sslmode={} connect_timeout={}",
            self.dbname,
            self.host,
            self.user,
            self.port,
            self.password,
            self.sslmode,
            self.connect_timeout
        )
    }
}
#[derive(Args, Debug)]
pub struct DataDirectoryCheckArgs {
    /// connection timeout in seconds
    #[arg(short, long, default_value = "5", value_hint = clap::ValueHint::CommandString)]
    pub connect_timeout: String,

    /// optional database used to connect to postgres
    #[arg(short, long, default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub dbname: String,

    #[arg(long, env = "PGHOST", default_value = "localhost", value_hint = clap::ValueHint::CommandString)]
    pub host: String,

    #[arg(short = 'P', long, env = "PGPASSWORD", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub password: String,

    #[arg(short, long, env = "PGPORT", default_value = "5432", value_hint = clap::ValueHint::CommandString)]
    pub port: String,

    #[arg(long, default_value_t = SslMode::Prefer)]
    pub sslmode: SslMode,

    /// user for connecting to postgres.
    #[arg(short, long, env = "PGUSER", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub user: String,

    /// expected data directory.
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub data_directory: String,

    /// optional remedy if command fails
    #[arg(long, value_hint = clap::ValueHint::CommandString)]
    pub remedy: Option<String>,
}

impl fmt::Display for DataDirectoryCheckArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dbname={} host={} user={} port={} password={} sslmode={} connect_timeout={}",
            self.dbname,
            self.host,
            self.user,
            self.port,
            self.password,
            self.sslmode,
            self.connect_timeout
        )
    }
}

#[derive(Args, Debug)]
pub struct DatabaseCheckArgs {
    /// connection timeout in seconds
    #[arg(short, long, default_value = "5", value_hint = clap::ValueHint::CommandString)]
    pub connect_timeout: String,

    #[arg(short, long, default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub dbname: String,

    #[arg(long, env = "PGHOST", default_value = "localhost", value_hint = clap::ValueHint::CommandString)]
    pub host: String,

    #[arg(short = 'P', long, env = "PGPASSWORD", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub password: String,

    #[arg(short, long, env = "PGPORT", default_value = "5432", value_hint = clap::ValueHint::CommandString)]
    pub port: String,

    #[arg(long, default_value_t = SslMode::Prefer)]
    pub sslmode: SslMode,

    /// user for connecting to postgres.
    #[arg(short, long, env = "PGUSER", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub user: String,

    /// optional remedy if command fails
    #[arg(long, value_hint = clap::ValueHint::CommandString)]
    pub remedy: Option<String>,
}

impl fmt::Display for DatabaseCheckArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "host={} user={} port={} password={} sslmode={} connect_timeout={}",
            self.host, self.user, self.port, self.password, self.sslmode, self.connect_timeout
        )
    }
}

#[derive(Args, Debug)]
pub struct RoleCheckArgs {
    /// connection timeout in seconds
    #[arg(short, long, default_value = "5", value_hint = clap::ValueHint::CommandString)]
    pub connect_timeout: String,

    /// optional database used to connect to postgres
    #[arg(short, long, default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub dbname: String,

    #[arg(long, env = "PGHOST", default_value = "localhost", value_hint = clap::ValueHint::CommandString)]
    pub host: String,

    #[arg(short = 'P', long, env = "PGPASSWORD", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub password: String,

    #[arg(short, long, env = "PGPORT", default_value = "5432", value_hint = clap::ValueHint::CommandString)]
    pub port: String,

    #[arg(long, default_value_t = SslMode::Prefer)]
    pub sslmode: SslMode,

    /// user for connecting to postgres.
    #[arg(short, long, env = "PGUSER", default_value = "postgres", value_hint = clap::ValueHint::CommandString)]
    pub user: String,

    /// role to check for existence.
    #[arg(short, long, value_hint = clap::ValueHint::CommandString)]
    pub role: String,

    /// optional remedy if command fails
    #[arg(long, value_hint = clap::ValueHint::CommandString)]
    pub remedy: Option<String>,
}

impl fmt::Display for RoleCheckArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dbname={} host={} user={} port={} password={} sslmode={} connect_timeout={}",
            self.dbname,
            self.host,
            self.user,
            self.port,
            self.password,
            self.sslmode,
            self.connect_timeout
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
