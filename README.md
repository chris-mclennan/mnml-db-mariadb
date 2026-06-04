# mnml-db-mariadb

MariaDB / MySQL query playground for [mnml](https://mnml.sh) — terminal
TUI with multiple saved connections and a results table. Sibling to
[mnml-db-postgres](https://github.com/chris-mclennan/mnml-db-postgres);
same shape, swap in `mysql_async` for the wire protocol.

The MySQL wire protocol is shared between MariaDB and MySQL servers
since they forked, so this binary points at either without code
changes. Connection string just needs `mysql://` scheme.

## Install

```sh
cargo install --git https://github.com/chris-mclennan/mnml-db-mariadb mnml-db-mariadb
```

## Setup

1. **Run once** to scaffold the config:
   ```sh
   mnml-db-mariadb
   ```
   Writes `~/.config/mnml-db-mariadb.toml` and exits. `chmod 600` it.

2. **Edit `[[connections]]`** with your DSNs:
   ```toml
   [[connections]]
   name = "prod-api"
   dsn  = "mysql://api_readonly:${PROD_DB_PASS}@db.prod.example.com:3306/api"
   ```
   `${ENV_VAR}` references are expanded at load time.

3. **Re-run** — TUI launches; type a query, `Ctrl+Enter` to run.

## Keys

Same as mnml-db-postgres:

| Chord                | Action                                            |
|----------------------|---------------------------------------------------|
| `Ctrl+Enter` / `F5`  | Run the current query                             |
| `Alt+1`-`Alt+9`      | Switch to that connection                         |
| `Ctrl+U`             | Clear the query buffer                            |
| `Ctrl+↑/↓` / `Ctrl+P/N` | Move selection in the results table             |
| `PgUp` / `PgDn`      | Jump 10 rows                                      |
| `Ctrl+Home` / `Ctrl+End` | Top / bottom of results                       |
| `R` (uppercase)      | Double `row_limit` for the next run               |
| `q` / `Esc` / `Ctrl+C` | Quit                                            |

## Safety: read-only by convention

Use a **read-only MariaDB/MySQL user** for production DSNs. This binary
runs whatever statement you type.

## License

MIT.
