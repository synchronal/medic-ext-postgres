#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(iter_intersperse)]

use crate::cli::app;
use medic_lib::CheckResult::{self, CheckError, CheckOk};
use postgres::{Client, NoTls};

pub mod cli;
pub mod env;

pub fn check_running(args: app::DefaultConnectionArgs) -> CheckResult {
    match Client::connect(&env::expand_string(format!("{args}")), NoTls) {
        Ok(_) => CheckOk,
        Err(err) => CheckError(
            "Unable to connect to Postgres".into(),
            Some(format!("{err}")),
            Some(format!("Connection params: {args}")),
            args.remedy.or(Some("pg-start".into())),
        ),
    }
}

pub fn database_exists(args: app::DatabaseCheckArgs) -> CheckResult {
    match Client::connect(&env::expand_string(format!("{args}")), NoTls) {
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
                    format!("Postgres database `{}` does not exist.", args.dbname),
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
            Some(format!("Connection params: {args}")),
            Some("pg-start".into()),
        ),
    }
}

pub fn role_exists(args: app::RoleCheckArgs) -> CheckResult {
    match Client::connect(&env::expand_string(format!("{args}")), NoTls) {
        Ok(mut client) => match client.query("SELECT usename FROM pg_catalog.pg_user", &[]) {
            Ok(rows) => {
                let roles = rows.iter().map(|row| row.get(0)).collect::<Vec<String>>();
                for name in &roles {
                    if name == &args.role {
                        return CheckOk;
                    }
                }
                let role_list: String = roles.join("\r\n");
                let default_remedy =
                    env::expand_string(format!("createuser -s {} -U {}", args.role, args.user));
                CheckError(
                    format!("Postgres role `{}` does not exist.", args.role),
                    Some(format!("Roles:\r\n{}", role_list)),
                    None,
                    args.remedy.or(Some(default_remedy)),
                )
            }
            Err(err) => CheckError(
                "Unable to retrieve role list".into(),
                Some(format!("{err}")),
                None,
                None,
            ),
        },
        Err(err) => CheckError(
            "Unable to connect to Postgres".into(),
            Some(format!("{err}")),
            Some(format!("Connection params: {args}")),
            Some("pg-start".into()),
        ),
    }
}
