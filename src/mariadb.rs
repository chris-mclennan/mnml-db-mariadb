//! mysql_async wrapper. v0.1 uses the text protocol (raw rows
//! coerced via `from_value::<String>`) so we render every cell
//! verbatim without per-type formatting. v0.2 will move to typed
//! decoding with rich-type display.

use anyhow::{Context, Result};
use mysql_async::prelude::*;
use mysql_async::{Conn, Row};

/// Open a MySQL/MariaDB connection from a DSN
/// (`mysql://user:pass@host:port/db`).
pub async fn connect(dsn: &str) -> Result<Conn> {
    let opts = mysql_async::OptsBuilder::from_opts(
        mysql_async::Opts::from_url(dsn).context("parsing DSN")?,
    );
    let conn = Conn::new(opts).await.context("connecting to MariaDB")?;
    Ok(conn)
}

/// Returned rows + column headers, ready for the TUI's table widget.
#[derive(Debug, Clone, Default)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    /// Time the query took to complete.
    pub elapsed: std::time::Duration,
    /// Total rows returned by the server (may exceed `rows.len()`
    /// when the client-side cap truncates the view).
    pub server_row_count: usize,
    /// True when `rows.len() < server_row_count` (cap hit).
    pub truncated: bool,
}

/// Run a query against `conn`. Caps the materialized result at
/// `row_limit` to keep an accidental `SELECT *` from a 10M-row
/// table from buffering forever. NULLs render as the literal `NULL`.
pub async fn run_query(conn: &mut Conn, sql: &str, row_limit: u32) -> Result<QueryResult> {
    let start = std::time::Instant::now();
    let result: Vec<Row> = conn.query(sql).await.context("running query")?;
    let elapsed = start.elapsed();

    let columns: Vec<String> = result
        .first()
        .map(|r| {
            r.columns_ref()
                .iter()
                .map(|c| c.name_str().to_string())
                .collect()
        })
        .unwrap_or_default();

    let server_row_count = result.len();
    let take = (row_limit as usize).min(result.len());
    let truncated = result.len() > take;
    let rows: Vec<Vec<String>> = result
        .into_iter()
        .take(take)
        .map(|row| {
            (0..row.len())
                .map(|i| match row.as_ref(i) {
                    Some(mysql_async::Value::NULL) | None => "NULL".to_string(),
                    Some(v) => match mysql_async::from_value_opt::<String>(v.clone()) {
                        Ok(s) => s,
                        Err(_) => format!("{v:?}"),
                    },
                })
                .collect()
        })
        .collect();

    Ok(QueryResult {
        columns,
        rows,
        elapsed,
        server_row_count,
        truncated,
    })
}
