use prettytable::Table;
use prettytable::format::TableFormat;
use rusqlite::Connection;
use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::{Stdout, stdout};
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Default, Clone, Copy)]
pub enum TableMode {
    Ascii,
    #[default]
    Box,
    Csv,
    Column,
    Html,
    Insert,
    Json,
    Line,
    List,
    Markdown,
    Quote,
    Table,
    Tabs,
    Tcl,
}

#[derive(Debug)]
pub struct UnrecognizedTableMode;

impl TryFrom<&str> for TableMode {
    type Error = UnrecognizedTableMode;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ascii" => Ok(TableMode::Ascii),
            "box" => Ok(TableMode::Box),
            "csv" => Ok(TableMode::Csv),
            "column" => Ok(TableMode::Column),
            "html" => Ok(TableMode::Html),
            "insert" => Ok(TableMode::Insert),
            "json" => Ok(TableMode::Json),
            "line" => Ok(TableMode::Line),
            "list" => Ok(TableMode::List),
            "markdown" => Ok(TableMode::Markdown),
            "quote" => Ok(TableMode::Quote),
            "table" => Ok(TableMode::Table),
            "tabs" => Ok(TableMode::Tabs),
            "tcl" => Ok(TableMode::Tcl),
            _ => Err(UnrecognizedTableMode),
        }
    }
}

impl From<TableMode> for &str {
    fn from(value: TableMode) -> Self {
        match value {
            TableMode::Ascii => "ascii",
            TableMode::Box => "box",
            TableMode::Csv => "csv",
            TableMode::Column => "column",
            TableMode::Html => "html",
            TableMode::Insert => "insert",
            TableMode::Json => "json",
            TableMode::Line => "line",
            TableMode::List => "list",
            TableMode::Markdown => "markdown",
            TableMode::Quote => "quote",
            TableMode::Table => "table",
            TableMode::Tabs => "tabs",
            TableMode::Tcl => "tcl",
        }
    }
}

#[derive(Debug)]
pub struct InvalidRepresentation;

impl TryFrom<TableMode> for TableFormat {
    type Error = InvalidRepresentation;
    fn try_from(value: TableMode) -> Result<Self, Self::Error> {
        match value {
            TableMode::Tabs => Ok(*crate::consts::TABS),
            TableMode::Csv => Ok(*crate::consts::CSV),
            TableMode::Column => Ok(*crate::consts::COLUMN),
            TableMode::Markdown => Ok(*crate::consts::MARKDOWN),
            TableMode::List => Ok(*crate::consts::LIST),
            TableMode::Box => Ok(*crate::consts::BOX),
            TableMode::Table => Ok(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE),
            TableMode::Line => Ok(*crate::consts::LINE),
            _ => Err(InvalidRepresentation),
        }
    }
}

pub enum Output {
    BufferedStdout(BufWriter<Stdout>),
    BufferedFile(BufWriter<File>),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::BufferedStdout(_) => f.pad("stdout"),
            Output::BufferedFile(_) => f.pad("file"),
        }
    }
}

impl Output {
    pub fn print_prettytable(&mut self, tbl: &mut Table) {
        match self {
            Output::BufferedStdout(buf_stdout) => {
                let _ = tbl.print(buf_stdout).expect("unable to print all bytes");
            }
            Output::BufferedFile(buf_file) => {
                let _ = tbl.print(buf_file).expect("unable to print all bytes");
            }
        }
    }

    pub fn flush(&mut self) {
        match self {
            Output::BufferedStdout(buf_stdout) => buf_stdout
                .flush()
                .expect("unable to flush writing to stdout"),
            Output::BufferedFile(buf_file) => {
                buf_file.flush().expect("unable to flush writing to file")
            }
        }
    }
}

pub struct Context {
    pub(crate) conn: Rc<RefCell<Connection>>,
    pub(crate) output: Output,
    pub(crate) mode: TableMode,
    pub(crate) command: Option<String>,
    pub(crate) cwd: PathBuf,
    pub(crate) with_header: bool,
    pub(crate) with_echo: bool,
    pub(crate) null_value_repr: Option<String>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            conn: Rc::new(RefCell::new(
                Connection::open_in_memory()
                    .expect("unable to establish an in-memory database connection"),
            )),
            output: Output::BufferedStdout(BufWriter::new(stdout())),
            mode: TableMode::Box,
            command: None,
            cwd: std::env::current_dir().expect("cwd may not exists or insuffiecient permission"),
            with_header: false,
            with_echo: false,
            null_value_repr: None,
        }
    }
}

impl Context {
    pub fn set_conn(&mut self, path: String) {
        self.cwd.push(&path);

        *self.conn.borrow_mut() =
            Connection::open(&self.cwd).expect("unable to establish a database connection");
        self.cwd.pop();
    }
    pub fn set_output(&mut self, path: String) {
        self.cwd.push(&path);

        let dump_file = File::open(&self.cwd).expect("unable to open file");
        let bufwriter = BufWriter::new(dump_file);
        self.output = Output::BufferedFile(bufwriter)
    }

    pub fn set_mode(&mut self, mode: String) {
        let mode = &mode as &str;
        let result = TableMode::try_from(mode);
        match result {
            Ok(new_mode) => self.mode = new_mode,
            Err(_) => eprintln!("unrecognized table"),
        }
    }

    pub fn set_command(&mut self, command: String) {
        self.command = Some(command);
    }
    pub fn set_with_header(&mut self) {
        self.with_header = !self.with_header;
    }
    pub fn set_with_echo(&mut self) {
        self.with_echo = !self.with_echo;
    }
    pub fn set_null_value(&mut self, null_value: String) {
        self.null_value_repr = Some(null_value);
    }
}
