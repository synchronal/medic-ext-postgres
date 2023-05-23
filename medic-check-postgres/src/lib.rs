#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(iter_intersperse)]

use crate::cli::app::DatabaseCheckArgs;
use crate::cli::app::DefaultConnectionArgs;
use medic_lib::CheckResult::{self, CheckError, CheckOk};
use postgres::{Client, NoTls};

pub mod cli;

pub fn check_running(args: DefaultConnectionArgs) -> CheckResult {
    match Client::connect(&format!("{args}"), NoTls) {
        Ok(_) => CheckOk,
        Err(err) => CheckError(
            "Unable to connect to Postgres".into(),
            Some(format!("{err}")),
            None,
            args.remedy.or(Some("pg-start".into())),
        ),
    }
}

pub fn database_exists(args: DatabaseCheckArgs) -> CheckResult {
    match Client::connect(&format!("{args}"), NoTls) {
        Ok(mut client) => match client.query("SELECT datname FROM pg_database", &[]) {
            Ok(rows) => {
                let databases = rows.iter().map(|row| row.get(0)).collect::<Vec<String>>();
                for name in &databases {
                    if name == &args.dbname {
                        return CheckOk;
                    }
                }
                let database_list: String = databases.join("\r\n");
                CheckError(
                    format!("Database {} not found.", args.dbname),
                    Some(format!("Databases:\r\n{}", database_list)),
                    None,
                    args.remedy,
                )
            }
            Err(err) => CheckError(
                "Unable to retrieve database list".into(),
                Some(format!("{err}")),
                None,
                None,
            ),
        },
        Err(err) => CheckError(
            "Unable to connect to Postgres".into(),
            Some(format!("{err}")),
            None,
            Some("pg-start".into()),
        ),
    }
}
