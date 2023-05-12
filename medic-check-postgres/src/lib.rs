#![cfg_attr(feature = "strict", deny(warnings))]

use crate::cli::app::ConnectionArgs;
use medic_lib::CheckResult::{self, CheckError, CheckOk};
use postgres::{Client, NoTls};

pub mod cli;

pub fn check_running(args: ConnectionArgs) -> CheckResult {
    match Client::connect(&format!("{args}"), NoTls) {
        Ok(_) => CheckOk,
        Err(err) => CheckError(
            "Unable to connect to Postgres".into(),
            Some(format!("{err}")),
            None,
            Some("pg-start".into()),
        ),
    }
}
