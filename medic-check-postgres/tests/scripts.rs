//! Runs the shell scripts in share/bin against stubbed brew, medic-bash,
//! postgres, pg_ctl and pgrep, recording the commands they would run.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const SCRIPTS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../share/bin");

struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    fn new(name: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("medic-ext-postgres-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        // Scripts see the physical path via `pwd` (macOS temp dirs are symlinks).
        let root = fs::canonicalize(root).unwrap();
        let sandbox = Self { root };

        let bin = sandbox.bin();
        let log = sandbox.log();
        let lib = sandbox.path("brew/share/medic-bash");
        fs::create_dir_all(&bin).unwrap();
        fs::create_dir_all(&lib).unwrap();
        fs::create_dir_all(sandbox.cwd()).unwrap();
        fs::write(&log, "").unwrap();

        let brew = sandbox.path("brew");
        executable(&bin.join("brew"), &format!("echo '{}'", brew.display()));
        executable(&bin.join("postgres"), "");
        executable(&bin.join("medic-pg-list"), "");
        executable(
            &bin.join("pg_ctl"),
            &format!("echo \"pg_ctl $*\" >> '{}'", log.display()),
        );
        executable(
            &bin.join("pgrep"),
            r#"[[ -n "${PGREP_OUTPUT}" ]] && printf '%s\n' "${PGREP_OUTPUT}"; true"#,
        );
        // GNU realpath rejects `<file>/..`, which the scripts pass it.
        executable(&bin.join("realpath"), r#"dirname "$(dirname "$1")""#);

        fs::write(lib.join("cecho.bash"), "cecho() { echo \"$@\" >&2; }\n").unwrap();
        fs::write(lib.join("check.bash"), "localcopy() { :; }\n").unwrap();
        fs::write(lib.join("confirm.bash"), "confirm() { return 1; }\n").unwrap();
        fs::write(
            lib.join("step.bash"),
            format!(
                "step() {{ echo \"$2\" >> '{}'; }}\nstep_header() {{ :; }}\n",
                log.display()
            ),
        )
        .unwrap();

        sandbox
    }

    fn path(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }

    fn bin(&self) -> PathBuf {
        self.path("bin")
    }

    fn cwd(&self) -> PathBuf {
        self.path("cwd")
    }

    fn log(&self) -> PathBuf {
        self.path("commands.log")
    }

    /// Runs a script and returns the commands it ran through `step` or `pg_ctl`.
    fn run(&self, script: &str, args: &[&str], env: &[(&str, String)]) -> Vec<String> {
        let output = Command::new("bash")
            .arg(Path::new(SCRIPTS).join(script))
            .args(args)
            .current_dir(self.cwd())
            .env_clear()
            .env("PATH", format!("{}:/usr/bin:/bin", self.bin().display()))
            .envs(env.iter().cloned())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{script} failed\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        fs::read_to_string(self.log())
            .unwrap()
            .lines()
            .map(String::from)
            .collect()
    }

    fn start(&self, env: &[(&str, String)]) -> Vec<String> {
        let mut env = env.to_vec();
        env.push(("MEDIC_APPLY_REMEDIES", "true".into()));
        self.run("medic-pg-start", &[], &env)
    }

    /// Runs medic-pg-stop with postgres apparently running at `running`.
    fn stop<P: AsRef<Path>>(&self, env: &[(&str, String)], running: &[P]) -> Vec<String> {
        let pgrep = running
            .iter()
            .enumerate()
            .map(|(i, dir)| {
                format!(
                    "{} /opt/pg/bin/postgres -D {}",
                    100 + i,
                    dir.as_ref().display()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let mut env = env.to_vec();
        env.push(("PGREP_OUTPUT", pgrep));
        self.run("medic-pg-stop", &["-q"], &env)
    }

    fn expected_start(&self, datadir: &Path, logfile: &Path) -> Vec<String> {
        vec![
            format!("pg_ctl -D {} initdb", datadir.display()),
            format!(
                "{}/pg_ctl -D {} -l {} -o '-p 5432' start",
                self.bin().display(),
                datadir.display(),
                logfile.display()
            ),
        ]
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn executable(path: &Path, body: &str) {
    fs::write(path, format!("#!/usr/bin/env bash\n{body}\n")).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

fn env<P: AsRef<Path>>(pairs: &[(&'static str, P)]) -> Vec<(&'static str, String)> {
    pairs
        .iter()
        .map(|(k, v)| (*k, v.as_ref().display().to_string()))
        .collect()
}

#[test]
fn start_defaults_to_priv_postgres() {
    let s = Sandbox::new("start-default");
    let pgdir = s.cwd().join("priv/postgres");

    let commands = s.start(&[]);

    assert_eq!(
        commands,
        s.expected_start(&pgdir.join("data"), &pgdir.join("logfile"))
    );
    assert!(pgdir.join("data").is_dir());
}

#[test]
fn start_uses_pg_root() {
    let s = Sandbox::new("start-pg-root");
    let root = s.path("root");

    let commands = s.start(&env(&[("PG_ROOT", &root)]));

    assert_eq!(
        commands,
        s.expected_start(&root.join("data"), &root.join("logfile"))
    );
    assert!(root.join("data").is_dir());
}

#[test]
fn start_uses_pgdata_with_log_in_parent() {
    let s = Sandbox::new("start-pgdata");
    let pgdata = s.path("custom/pgdata");

    let commands = s.start(&env(&[("PGDATA", &pgdata)]));

    assert_eq!(
        commands,
        s.expected_start(&pgdata, &s.path("custom/logfile"))
    );
    assert!(pgdata.is_dir());
}

#[test]
fn start_prefers_pgdata_and_logs_to_pg_root() {
    let s = Sandbox::new("start-both");
    let pgdata = s.path("custom/pgdata");
    let root = s.path("root");

    let commands = s.start(&env(&[("PGDATA", &pgdata), ("PG_ROOT", &root)]));

    assert_eq!(commands, s.expected_start(&pgdata, &root.join("logfile")));
    assert!(pgdata.is_dir());
    assert!(!root.join("data").exists());
}

fn stop_command(datadir: &Path) -> Vec<String> {
    vec![format!("/opt/pg/bin/pg_ctl -D {} stop", datadir.display())]
}

#[test]
fn stop_defaults_to_priv_postgres() {
    let s = Sandbox::new("stop-default");
    let datadir = s.cwd().join("priv/postgres/data");

    let commands = s.stop(&[], &[&s.path("decoy"), &datadir]);

    assert_eq!(commands, stop_command(&datadir));
}

#[test]
fn stop_uses_pg_root() {
    let s = Sandbox::new("stop-pg-root");
    let root = s.path("root");
    let datadir = root.join("data");

    let commands = s.stop(&env(&[("PG_ROOT", &root)]), &[&s.path("decoy"), &datadir]);

    assert_eq!(commands, stop_command(&datadir));
}

#[test]
fn stop_prefers_pgdata_over_pg_root() {
    let s = Sandbox::new("stop-both");
    let pgdata = s.path("custom/pgdata");
    let root = s.path("root");

    let commands = s.stop(
        &env(&[("PGDATA", &pgdata), ("PG_ROOT", &root)]),
        &[&root.join("data"), &pgdata],
    );

    assert_eq!(commands, stop_command(&pgdata));
}

#[test]
fn stop_is_quiet_when_nothing_runs_at_pgdata() {
    let s = Sandbox::new("stop-none");
    let pgdata = s.path("custom/pgdata");

    let commands = s.stop(&env(&[("PGDATA", &pgdata)]), &[&s.path("decoy")]);

    assert!(commands.is_empty(), "{commands:?}");
}
