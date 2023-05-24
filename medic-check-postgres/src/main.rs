use medic_check_postgres::cli::{CliArgs, Command};
use medic_lib::CheckResult::{self, CheckOk};

fn main() -> CheckResult {
    let cli = CliArgs::new();

    match cli.command {
        Command::DataDirectory(args) => medic_check_postgres::data_directory(args)?,
        Command::DatabaseExists(args) => medic_check_postgres::database_exists(args)?,
        Command::RoleExists(args) => medic_check_postgres::role_exists(args)?,
        Command::Running(args) => medic_check_postgres::check_running(args)?,
    }

    CheckOk
}
