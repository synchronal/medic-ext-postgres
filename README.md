# Medic Postgres

An extension pack for using [medic](https://github.com/synchronal/medic-rs)
with Postgres.

```toml
[doctor]

checks = [
  # ...
  { check = "postgres", command = "running", args = { remedy = "bin/dev/db-start", user = "${USER}"} },
  { check = "postgres", command = "role-exists", args = { role = "postgres", user = "${USER}" } },
  { check = "postgres", command = "data-directory", args = { data-directory = "./priv/postgres/data", remedy = "bin/dev/db-restart" } },
  { check = "postgres", command = "database-exists", args = { dbname = "my_app_dev", remedy = "mix ecto.reset" } },
  # ...
]
```

## medic-check-postgres

Checks for whether a Postgres database is set up for development.

All checks take the following arguments:
- `dbname` - string - name of the database to connect to when accessing Postgres.
- `connect-timeout` - integer - seconds to wait before failing to connect.
- `host` - string - defaults to `localhost` or to the value of `PGHOST`.
- `password` - string - defaults to `postgres` or to the value of `PGPASSWORD`.
- `port` - integer - defaults to `5432` or to the value of `PGPORT`.
- `sslmode` - string - defaults to `prefer`.
- `user` - string - defaults to `postgres` or the value of `PGUSER`.
- `remedy` - string - an optional remedy which will override any default remedies.

### data-directory

Where is the data directory for the given Postgres instances?

Required arguments:
- `data-directory` - absolute or relative path to the expected data directory.

```shell
medic-check-postgres data-directory --data-directory <path>
```

### database-exists

Does the given database exist?

Required arguments:
- `dbname`

```shell
medic-check-postgres database-exists --dbname <name>
```

### role-exists

Does the give role exists in the database?

Required arguments:
- `role` - the name of the role that should be present.

```shell
medic-check-postgres role-exists --role <name>
```

### running

Is postgres running on the given port? Note that this connects to the database in order to
run its check, so must be given valid credentials.

```shell
medic-check-postgres running
```
