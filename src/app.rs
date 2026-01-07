use crate::shqlite::Shqlite;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "shqlite", about = "A simple terminal `sqlite` client in Rust", long_about = None)]
pub struct App {
    /// FILENAME is the name of an SQLite database. A new database is created
    /// if the file does not previously exist. Defaults to :memory:.
    pub(crate) filename: Option<String>,

    /// set output mode
    #[arg(short, long, default_value = "boxed", value_parser = clap::builder::PossibleValuesParser::new([
        "ascii",
        "boxed",
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
    pub(crate) mode: String,

    /// read/process named files
    #[arg(short, long)]
    pub(crate) init: Option<String>,

    /// run "COMMAND" before reading stdin
    #[arg(short, long)]
    pub(crate) cmd: Option<String>,

    /// turn headers on or off
    #[arg(long, overrides_with = "_no_header")]
    pub(crate) header: bool,

    #[arg(long = "no-header")]
    pub(crate) _no_header: bool,
}

impl From<App> for Shqlite {
    fn from(value: App) -> Self {
        let mut shqlite = Self::default();

        shqlite.set_format(value.mode);

        if value.header {
            shqlite.set_header();
        }

        if let Some(cmd) = value.cmd {
            shqlite.set_command(cmd);
        }

        if let Some(output) = value.init {
            shqlite.set_output(output);
        }

        if let Some(filename) = value.filename {
            shqlite.set_db_conn(filename);
        }

        shqlite
    }
}
