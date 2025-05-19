# Change log

## Unreleased

## 2.0.0

- `medic-pg-stop` does not stop other PostgeSQL instances when in subshells, or quiet.
- `medic-pg-stop` automatically stops all other PostgreSQL instances when `--yes`.

## 1.2.0

- `medic-pg-start` automatically creates data directory when `MEDIC_APPLY_REMEDIES` is true.

## 1.1.1

- Fix `medic-pg-stop` to look for postgres running database relative to `PG_ROOT`.

## 1.1.0

- Scripts initialize database relative to `PG_ROOT`, if present, with a default of `./priv/postgres`.

## 1.0.0

- Scripts do not assume postgres is installed via `asdf`, but look in the current path for `postgres`.

## 0.1.1

- Include shell scripts in release
  - `medic-pg-list`
  - `medic-pg-restart`
  - `medic-pg-start`
  - `medic-pg-stop`

## 0.1.0

- Initial release.
- `medic-check-postgres`
  - `data-directory`
  - `database-exists`
  - `role-exists`
  - `running`
