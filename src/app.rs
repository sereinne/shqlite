use clap::Parser;

use crate::config::Context;

#[derive(Parser)]
#[command(name = "shqlite", version = "0.1.0", about = "terminal sqlite client written in rust", long_about = None)]
pub struct App {
    /// FILENAME is the name of an SQLite database. A new database is created
    /// if the file does not previously exist. Defaults to :memory:.
    filename: Option<String>,

    /// sets output mode to one out of these values (default: box)
    #[arg(short, long, default_value = "box", value_parser = clap::builder::PossibleValuesParser::new([
            "ascii",
            "box",
            "csv",
            "column",
            "html",
            "insert",
            "json",
            "line",
            "list",
            "markdown",
            "quote",
            "table",
            "tabs",
            "tcl"
        ]))]
    mode: String,

    /// read/process named sql file, by default is going to read input from stdout
    #[arg(short, long)]
    init: Option<String>,

    /// run "COMMAND" before reading stdin
    command: Option<String>,

    /// turn headers on or off
    #[arg(long, overrides_with = "_no_header")]
    header: bool,

    /// reciprocal of --header flag
    #[arg(long = "no-header")]
    _no_header: bool,

    /// print inputs before execution
    #[arg(short, long)]
    echo: bool,

    /// replace null values with something else
    #[arg(long = "null-value")]
    null_value: Option<String>,
}

impl From<App> for Context {
    fn from(value: App) -> Self {
        let mut ctx = Self::default();

        if value.echo {
            ctx.set_with_echo();
        }

        if value.header {
            ctx.set_with_header();
        }

        ctx.set_mode(value.mode);

        if let Some(conn) = value.filename {
            ctx.set_conn(conn);
        }

        if let Some(output_file) = value.init {
            ctx.set_output(output_file);
        }

        if let Some(cmd) = value.command {
            ctx.set_command(cmd);
        }

        if let Some(nv) = value.null_value {
            ctx.set_null_value(nv);
        }

        ctx
    }
}
