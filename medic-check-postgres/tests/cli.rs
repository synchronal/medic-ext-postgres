use std::process::{Command, Output};

fn data_directory(pgdata: Option<&str>, args: &[&str]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_medic-check-postgres"));
    cmd.arg("data-directory").args(args).env_remove("PGDATA");
    if let Some(pgdata) = pgdata {
        cmd.env("PGDATA", pgdata);
    }
    cmd.output().unwrap()
}

fn combined(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn data_directory_is_required_without_pgdata() {
    let output = data_directory(None, &[]);
    let out = combined(&output);

    assert_eq!(output.status.code(), Some(2), "{out}");
    assert!(out.contains("--data-directory"), "{out}");
}

#[test]
fn data_directory_help_mentions_pgdata() {
    let output = data_directory(None, &["--help"]);
    let out = combined(&output);

    assert!(out.contains("PGDATA"), "{out}");
}

#[test]
fn data_directory_defaults_to_pgdata() {
    // Nothing listens on port 1, so a successful parse ends in a connection error.
    let output = data_directory(
        Some("/tmp/pgdata"),
        &[
            "--host",
            "127.0.0.1",
            "--port",
            "1",
            "--connect-timeout",
            "1",
        ],
    );
    let out = combined(&output);

    assert_ne!(output.status.code(), Some(2), "{out}");
    assert!(out.contains("Unable to connect to Postgres"), "{out}");
}
