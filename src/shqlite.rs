use crate::consts::HELP_COMMANDS;
use crate::util;
use prettytable::format::Alignment;
use prettytable::format::TableFormat;
use prettytable::{Cell, Row, Table, row, table};
use rusqlite::Connection;
use rusqlite::config::DbConfig;
use rusqlite::ffi::{SQLITE_SOURCE_ID, SQLITE_VERSION};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::fs::File;
use std::io::Stdout;
use std::io::stdout;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, exit};
use std::vec;

pub enum Output {
    BufferedStdout(BufWriter<Stdout>),
    BufferedFile(BufWriter<File>),
}

impl Output {
    pub fn get_to_str(&self) -> &'static str {
        match self {
            Output::BufferedStdout(_) => "stdout",
            Output::BufferedFile(_) => "file",
        }
    }
    pub fn flush(&mut self) {
        match self {
            Output::BufferedStdout(out) => out.flush().expect("unable to flush"),
            Output::BufferedFile(file) => file.flush().expect("unable to flush"),
        }
    }

    pub fn print_json(&mut self, tbl: &mut Table, title: &Row) {
        let obj: &mut dyn Write = match self {
            Output::BufferedStdout(sout) => sout,
            Output::BufferedFile(file) => file,
        };

        let _ = writeln!(obj, "[\n\t");
        // count the rows in `tbl` (does not include the title)
        let rows = tbl.len();
        // get the length of the each row (we don't need to recalculate everytime we encounter a new row it is guaranteed)
        let cols = title.len();

        // iterating through each row in a table
        for row_idx in 0..rows {
            let _ = writeln!(obj, "\t{{").expect("unable to write to trait object");
            let curr_row = tbl.get_row(row_idx).unwrap();
            // iterating through each cell in a row
            for col_idx in 0..cols {
                // get a cell of a row
                let cel = curr_row.get_cell(col_idx).unwrap();
                // get a title of that cell by using its cell index `col_idx`
                let title_cel = title.get_cell(col_idx).unwrap();

                // don't forget to omit the trailing comma! for each cell
                if col_idx == cols - 1 {
                    let _ = writeln!(
                        obj,
                        "\t\t\"{}\": {}",
                        title_cel.get_content(),
                        cel.get_content()
                    )
                    .expect("unable to write to trait object");
                    continue;
                }
                let _ = writeln!(
                    obj,
                    "\t\t\"{}\": {},",
                    title_cel.get_content(),
                    cel.get_content()
                )
                .expect("unable to write to trait object");
            }
            // don't forget to omit the trailing comma! for each row
            if row_idx == rows - 1 {
                let _ = writeln!(obj, "\t}}").expect("unable to write to trait object");
                continue;
            }
            let _ = writeln!(obj, "\t}},").expect("unable to write to trait object");
        }
        let _ = writeln!(obj, "\t\n]").expect("unable to write to trait object");

        obj.flush().expect("unable to flush");
    }

    pub fn write_from_table(
        &mut self,
        tbl: &mut Table,
        mode: TableMode,
        with_header: bool,
        title: Row,
    ) {
        // in order to print it using json
        let cloned = title.clone();
        // set title based on `TableMode` and `with_header`
        // based on what i have tested with how .headers work it only applies to some table mode (list, csv and tabs) not all of them
        // by default it will have a title
        match mode {
            TableMode::List | TableMode::Csv | TableMode::Tabs => {
                if with_header {
                    tbl.set_titles(title);
                } else {
                    tbl.unset_titles();
                }
            }
            _ => tbl.set_titles(title),
        }

        // because `prettytable` crate has a method named `print_html` which is convenient
        // it is handled here because there is no `TableFormat` that returns a html-like structure table
        match self {
            Output::BufferedStdout(sout) => match mode {
                TableMode::Html => tbl.print_html(sout).expect("unable to print html"),
                TableMode::Json => self.print_json(tbl, &cloned),
                _ => {
                    let _ = tbl.print(sout).expect("unable to fully print");
                }
            },
            Output::BufferedFile(file) => match mode {
                TableMode::Html => tbl.print_html(file).expect("unable to print html"),
                TableMode::Json => self.print_json(tbl, &cloned),
                _ => {
                    let _ = tbl.print(file).expect("unable to fully print");
                }
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum TableMode {
    Ascii,
    #[default]
    Boxed,
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

impl TableMode {
    pub fn get_table_format(&self) -> TableFormat {
        match self {
            TableMode::Table => *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
            TableMode::Tabs => *crate::consts::TABS,
            TableMode::Csv => *crate::consts::CSV,
            TableMode::Column => *crate::consts::COLUMN,
            TableMode::Markdown => *crate::consts::MARKDOWN,
            TableMode::List => *crate::consts::LIST,
            TableMode::Boxed => *crate::consts::BOXED,
            _ => *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE,
        }
    }

    pub fn get_to_str(&self) -> &'static str {
        match self {
            TableMode::Ascii => "ascii",
            TableMode::Boxed => "boxed",
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

impl From<&'_ str> for TableMode {
    fn from(value: &'_ str) -> Self {
        match value {
            "ascii" => TableMode::Ascii,
            "boxed" => TableMode::Boxed,
            "csv" => TableMode::Csv,
            "column" => TableMode::Column,
            "html" => TableMode::Html,
            "insert" => TableMode::Insert,
            "json" => TableMode::Json,
            "line" => TableMode::Line,
            "list" => TableMode::List,
            "markdown" => TableMode::Markdown,
            "quote" => TableMode::Quote,
            "table" => TableMode::Table,
            "tabs" => TableMode::Tabs,
            "tcl" => TableMode::Tcl,
            _ => {
                println!("unknown type of mode, defaults to boxed");
                TableMode::Boxed
            }
        }
    }
}

#[derive(Default)]
pub enum ConnectionType {
    #[default]
    Memory,
    Persistent(String),
}

impl ConnectionType {
    pub fn get_to_str(&self) -> &str {
        match self {
            ConnectionType::Memory => ":memory:",
            ConnectionType::Persistent(path) => &path,
        }
    }
}

pub struct Shqlite {
    db_conn: Connection,
    conn_type: ConnectionType,
    output: Output,
    format: TableMode,
    command: Option<String>,
    cwd: PathBuf,
    with_header: bool,
    with_echo: bool,
    null_value: Option<String>,
}

impl Default for Shqlite {
    fn default() -> Self {
        Self {
            db_conn: Connection::open_in_memory()
                .expect("could not establish a temporary database connection"),
            conn_type: ConnectionType::Memory,
            output: Output::BufferedStdout(BufWriter::new(stdout())),
            format: TableMode::Boxed,
            command: None,
            cwd: std::env::current_dir().expect("cwd may not exists or insuffiecient permission"),
            with_header: false,
            with_echo: false,
            null_value: None,
        }
    }
}

impl Shqlite {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_db_conn(&mut self, db_file: String) {
        let path = Path::new(&db_file);
        if path.exists() {
            self.db_conn =
                Connection::open(path).expect("incompatible path or unable to open db file...");
            self.conn_type = ConnectionType::Persistent(db_file);
        }
    }

    pub fn set_output(&mut self, dump_file: String) {
        let path = Path::new(&dump_file);
        if path.exists() {
            let file = File::open(path).expect("could not open file");
            self.output = Output::BufferedFile(BufWriter::new(file));
        }
    }

    pub fn set_format(&mut self, mode: String) {
        self.format = TableMode::from(&mode as &str);
    }

    pub fn set_command(&mut self, cmd: String) {
        self.command = Some(cmd);
    }

    pub fn set_header(&mut self) {
        self.with_header = true;
    }

    pub fn start_repl(&mut self) -> rustyline::Result<()> {
        let mut editor = DefaultEditor::new()?;

        if let Some(cmd) = self.command.clone() {
            self.handle_user_input(&cmd);
        }

        loop {
            let input = editor.readline("shqlite> ");
            match input {
                Ok(user_input) => self.handle_user_input(&user_input),
                Err(err) => Self::handle_readline_err(err),
            }
        }
    }

    fn handle_user_input(&mut self, user_input: &str) {
        if user_input.starts_with(".") {
            self.execute_dot_command(user_input);
        } else {
            self.execute_user_query(user_input);
        }
    }
    fn handle_readline_err(err: ReadlineError) {
        match err {
            ReadlineError::Eof => {
                println!("Ctrl-D Bye!");
                exit(0)
            }
            ReadlineError::Interrupted => {
                println!("Ctrl-C Bye!");
                exit(0)
            }
            _ => {
                println!("ERROR: {}", err);
                exit(1)
            }
        }
    }

    fn execute_user_query(&mut self, query: &str) {
        let mut stmt = self
            .db_conn
            .prepare(query)
            .expect("unable to create a prepared statement");
        let col_count = stmt.column_count();

        if col_count == 0 {
            stmt.execute([])
                .expect("unable to execute with a prepared statement");

            return;
        }

        let row_title = util::query_colname_as_tbl_rows(&stmt, col_count);
        let row_data =
            util::query_data_as_tbl_rows(&mut stmt, col_count, false, self.null_value.as_deref());

        // table to be displayed as a result of a query
        let mut table = Table::init(row_data);

        // set format based on `TableMode`
        let mode = self.format.get_table_format();
        table.set_format(mode);

        // write table data to stdout or a file
        self.output
            .write_from_table(&mut table, self.format, self.with_header, row_title);
        // flush all remaining content of the `BufWriter` to ensure all are written
        self.output.flush();
    }

    fn execute_dot_command(&mut self, user_input: &str) {
        let dot_cmd_with_args = user_input.split(" ").collect::<Vec<_>>();
        let dot_cmd: &str = dot_cmd_with_args[0];
        let dot_cmd_args = &dot_cmd_with_args[1..];

        if self.with_echo {
            match &mut self.output {
                Output::BufferedStdout(_) => println!("{}", user_input),
                Output::BufferedFile(file) => {
                    let _ = writeln!(file, "{}", user_input);
                }
            }
        }

        match dot_cmd {
            ".archive" => Self::dot_archive(dot_cmd_args),
            ".auth" => Self::dot_auth(dot_cmd_args),
            ".backup" => self.dot_backup(dot_cmd_args),
            ".bail" => Self::dot_bail(dot_cmd_args),
            ".cd" => self.dot_cd(dot_cmd_args),
            ".changes" => Self::dot_changes(dot_cmd_args),
            ".check" => Self::dot_check(dot_cmd_args),
            ".clone" => Self::dot_clone(dot_cmd_args),
            ".connection" => Self::dot_connection(dot_cmd_args),
            ".crlf" => Self::dot_crlf(dot_cmd_args),
            ".databases" => self.dot_databases(dot_cmd_args),
            ".dbconfig" => self.dot_dbconfig(),
            ".dbinfo" => Self::dot_dbinfo(dot_cmd_args),
            ".dbtotxt" => Self::dot_dbtotxt(dot_cmd_args),
            ".dump" => self.dot_dump(dot_cmd_args),
            ".echo" => self.dot_echo(dot_cmd_args),
            ".eqp" => Self::dot_eqp(dot_cmd_args),
            ".excel" => Self::dot_excel(dot_cmd_args),
            ".exit" => Self::dot_exit(dot_cmd_args),
            ".expert" => Self::dot_expert(dot_cmd_args),
            ".explain" => Self::dot_explain(dot_cmd_args),
            ".filectrl" => Self::dot_filectrl(dot_cmd_args),
            ".fullschema" => Self::dot_fullschema(dot_cmd_args),
            ".headers" => self.dot_headers(dot_cmd_args),
            ".help" => self.dot_help(),
            ".import" => Self::dot_import(dot_cmd_args),
            ".imposter" => Self::dot_imposter(dot_cmd_args),
            ".indexes" => self.dot_indexes(dot_cmd_args),
            ".intck" => Self::dot_intck(dot_cmd_args),
            ".limit" => Self::dot_limit(dot_cmd_args),
            ".lint" => Self::dot_lint(dot_cmd_args),
            ".load" => Self::dot_load(dot_cmd_args),
            ".log" => Self::dot_log(dot_cmd_args),
            ".mode" => self.dot_mode(dot_cmd_args),
            ".nonce" => Self::dot_nonce(dot_cmd_args),
            ".nullvalue" => self.dot_nullvalue(dot_cmd_args),
            ".once" => Self::dot_once(dot_cmd_args),
            ".open" => self.dot_open(dot_cmd_args),
            ".output" => self.dot_output(dot_cmd_args),
            ".parameter" => Self::dot_parameter(dot_cmd_args),
            ".print" => self.dot_print(dot_cmd_args),
            ".progress" => Self::dot_progress(dot_cmd_args),
            ".prompt" => Self::dot_prompt(dot_cmd_args),
            ".quit" => Self::dot_quit(),
            ".read" => self.dot_read(dot_cmd_args),
            ".recover" => Self::dot_recover(dot_cmd_args),
            ".restore" => Self::dot_restore(dot_cmd_args),
            ".save" => self.dot_save(dot_cmd_args),
            ".scanstats" => Self::dot_scanstats(dot_cmd_args),
            ".schema" => self.dot_schema(dot_cmd_args),
            ".separator" => self.dot_separator(dot_cmd_args),
            ".session" => Self::dot_session(dot_cmd_args),
            ".sha3sum" => Self::dot_sha3sum(dot_cmd_args),
            ".shell" => self.dot_shell(dot_cmd_args),
            ".show" => self.dot_show(),
            ".stats" => Self::dot_stats(dot_cmd_args),
            ".system" => self.dot_system(dot_cmd_args),
            ".tables" => self.dot_tables(),
            ".timeout" => Self::dot_timeout(dot_cmd_args),
            ".timer" => Self::dot_timer(dot_cmd_args),
            ".trace" => Self::dot_trace(dot_cmd_args),
            ".unmodule" => Self::dot_unmodule(dot_cmd_args),
            ".version" => self.dot_version(),
            ".vfsinfo" => Self::dot_vfsinfo(dot_cmd_args),
            ".vfslist" => Self::dot_vfslist(dot_cmd_args),
            ".vfsname" => Self::dot_vfsname(dot_cmd_args),
            ".width" => Self::dot_width(dot_cmd_args),
            ".www" => Self::dot_www(dot_cmd_args),
            _ => eprintln!(
                "Error: unknown command or invalid arguments:  \"{}\". Enter \".help\" for help",
                dot_cmd
            ),
        }
    }

    fn dot_archive(_args: &[&str]) {
        todo!("WIP to implement dot_archive function")
    }
    fn dot_auth(_args: &[&str]) {
        todo!("WIP to implement dot_auth function")
    }
    fn dot_backup(&self, args: &[&str]) {
        self.dot_save(args);
    }
    fn dot_bail(_args: &[&str]) {
        todo!("WIP to implement dot_bail function")
    }
    fn dot_cd(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".cd needs an argument to run");
            return;
        }

        let path = PathBuf::from(args[0]);

        if path.is_absolute() {
            self.cwd = path;
        } else if path.is_relative() && path.is_dir() {
            self.cwd.push(path);
        }
    }
    fn dot_changes(_args: &[&str]) {
        todo!("WIP to implement dot_changes function")
    }
    fn dot_check(_args: &[&str]) {
        todo!("WIP to implement dot_check function")
    }
    fn dot_clone(_args: &[&str]) {
        todo!("WIP to implement dot_clone function")
    }
    fn dot_connection(_args: &[&str]) {
        todo!("WIP to implement dot_connection function")
    }
    fn dot_crlf(_args: &[&str]) {
        todo!("WIP to implement dot_crlf function")
    }
    fn dot_databases(&mut self, _args: &[&str]) {
        let sql = "SELECT seq , name , file FROM pragma_database_list";

        let mut stmt = self
            .db_conn
            .prepare(sql)
            .expect("unable to create prepared statement");

        let col_count = stmt.column_count();
        let rows_title = util::query_colname_as_tbl_rows(&stmt, col_count);
        let row_data =
            util::query_data_as_tbl_rows(&mut stmt, col_count, false, self.null_value.as_deref());

        let fmt = self.format.get_table_format();

        let mut tbl = Table::init(row_data);
        tbl.set_format(fmt);
        self.output
            .write_from_table(&mut tbl, self.format, self.with_header, rows_title);
        self.output.flush();
    }
    fn dot_dbconfig(&mut self) {
        let attach_create = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE)
            .expect("unable to fetch config named attach_create");
        let attach_write = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE)
            .expect("unable to fetch config named attach_write");
        let comments = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_COMMENTS)
            .expect("unable to fetch config named comments");
        let defensive = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_DEFENSIVE)
            .expect("unable to fetch config named defensive");
        let dqs_ddl = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_DQS_DDL)
            .expect("unable to fetch config named dqs_ddl");
        let dqs_dml = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_DQS_DML)
            .expect("unable to fetch config named dqs_dml");
        let enable_fkey = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY)
            .expect("unable to fetch config named enable_fkey");
        let enable_qpsg = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_QPSG)
            .expect("unable to fetch config named enable_qpsg");
        let enable_trigger = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_TRIGGER)
            .expect("unable to fetch config named enable_trigger");
        let enable_view = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_VIEW)
            .expect("unable to fetch config named enable_view");
        let fts3_tokenizer = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER)
            .expect("unable to fetch config named fts3_tokenizer");
        let legacy_alter_table = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_LEGACY_ALTER_TABLE)
            .expect("unable to fetch config named legacy_alter_table");
        let legacy_file_format = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_LEGACY_FILE_FORMAT)
            .expect("unable to fetch config named legacy_file_format");
        let no_ckpt_on_close = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE)
            .expect("unable to fetch config named no_ckpt_on_close");
        let reset_database = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_RESET_DATABASE)
            .expect("unable to fetch config named reset_database");
        let reverse_scan_order = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_REVERSE_SCANORDER)
            .expect("unable to fetch config named reverse_scan_order");
        let stmt_scanstatus = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_STMT_SCANSTATUS)
            .expect("unable to fetch config named stmt_scanstatus");
        let trigger_eqp = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_TRIGGER_EQP)
            .expect("unable to fetch config named trigger_eqp");
        let trusted_schema = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_TRUSTED_SCHEMA)
            .expect("unable to fetch config named trusted_schema");
        let writable_schema = self
            .db_conn
            .db_config(DbConfig::SQLITE_DBCONFIG_WRITABLE_SCHEMA)
            .expect("unable to fetch config name writable_schema");

        let mut tbl = table! {
                ["attach_create", util::bool_to_onoff(attach_create)],
                ["attach_write", util::bool_to_onoff(attach_write)],
                ["comments", util::bool_to_onoff(comments)],
                ["defensive", util::bool_to_onoff(defensive)],
                ["dqs_ddl", util::bool_to_onoff(dqs_ddl)],
                ["dqs_dml", util::bool_to_onoff(dqs_dml)],
                ["enable_fkey", util::bool_to_onoff(enable_fkey)],
                ["enable_qpsg", util::bool_to_onoff(enable_qpsg)],
                ["enable_trigger", util::bool_to_onoff(enable_trigger)],
                ["enable_view", util::bool_to_onoff(enable_view)],
                ["fts3_tokenizer", util::bool_to_onoff(fts3_tokenizer)],
                ["legacy_alter_table", util::bool_to_onoff(legacy_alter_table)],
                ["legacy_file_format", util::bool_to_onoff(legacy_file_format)],
                ["no_ckpt_on_close", util::bool_to_onoff(no_ckpt_on_close)],
                ["reset_database", util::bool_to_onoff(reset_database)],
                ["reverse_scan_order", util::bool_to_onoff(reverse_scan_order)],
                ["stmt_scanstatus", util::bool_to_onoff(stmt_scanstatus)],
                ["trigger_eqp", util::bool_to_onoff(trigger_eqp)],
                ["trusted_schema", util::bool_to_onoff(trusted_schema)],
                ["writable_schema", util::bool_to_onoff(writable_schema)]
        };

        let fmt = self.format.get_table_format();
        tbl.set_format(fmt);

        self.output.write_from_table(
            &mut tbl,
            self.format,
            self.with_header,
            row!["pragmas", "values"],
        );
        self.output.flush();
    }

    fn dot_dbinfo(_args: &[&str]) {
        todo!("WIP to implement dot_dbinfo function")
    }
    fn dot_dbtotxt(_args: &[&str]) {
        todo!("WIP to implement dot_dbtotxt function")
    }

    fn dot_dump(&mut self, _args: &[&str]) {
        util::write_generic_sql_stmt(
            &mut self.output,
            "PRAGMA foreign_keys=OFF;\nBEGIN TRANSACTION",
        );

        let sql =
            "SELECT name, type, sql FROM sqlite_schema WHERE  name NOT LIKE 'sqlite_%' ORDER BY 1";

        let mut stmt = self
            .db_conn
            .prepare(sql)
            .expect("unable to construct a prepared statement");

        let col_count = stmt.column_count();
        let row_data_str =
            util::query_data_as_str(&mut stmt, col_count, false, self.null_value.as_deref());

        for data in row_data_str.iter() {
            let item_name: &str = &data[0];
            let type_name: &str = &data[1];
            let sql: &str = &data[2];

            match type_name {
                "table" | "view" => {
                    // output the sql definition of that table
                    util::write_generic_sql_stmt(&mut self.output, sql);
                    // populate that table by querying `SELECT *`, parse its values and write it to stdout or a file
                    let sql = format!("SELECT * FROM {}", item_name);

                    let mut stmt = self.db_conn.prepare(&sql).expect(
                        "unable construct a second prepared statement when trying to dump db",
                    );
                    let col_count = stmt.column_count();

                    // data to be written for the VALUES for INSERT INTO
                    let insert_rows = util::query_data_as_str(
                        &mut stmt,
                        col_count,
                        true,
                        self.null_value.as_deref(),
                    );

                    for insert_row in insert_rows.iter() {
                        util::write_insert_stmt(&mut self.output, item_name, insert_row);
                    }
                }
                "index" => {
                    // output the sql of an index
                    util::write_generic_sql_stmt(&mut self.output, sql);
                }
                _ => {
                    // unsupported format
                    println!("unsupported format");
                }
            }
        }

        util::write_generic_sql_stmt(&mut self.output, "COMMIT");
        self.output.flush();
    }

    fn dot_echo(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".echo needs an argument...");
            return;
        }

        let arg = args[0];
        self.with_echo = util::onoff_to_bool(arg);
    }
    fn dot_eqp(_args: &[&str]) {
        todo!("WIP to implement dot_eqp function")
    }
    fn dot_excel(_args: &[&str]) {
        todo!("WIP to implement dot_excel function")
    }
    fn dot_exit(args: &[&str]) {
        if args.is_empty() {
            exit(0);
        }
        let exit_code = args[0].parse::<i32>().expect("unable to parse integer");
        exit(exit_code);
    }
    fn dot_expert(_args: &[&str]) {
        todo!("WIP to implement dot_expert function")
    }
    fn dot_explain(_args: &[&str]) {
        todo!("WIP to implement dot_explain function")
    }
    fn dot_filectrl(_args: &[&str]) {
        todo!("WIP to implement dot_filectrl function")
    }
    fn dot_fullschema(_args: &[&str]) {
        todo!("WIP to implement dot_fullschema function")
    }
    fn dot_headers(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".headers");
            return;
        }

        let arg = args[0];
        self.with_header = util::onoff_to_bool(arg);
    }
    fn dot_help(&mut self) {
        let mut tbl = Table::new();

        for [cmd, arg, desc] in HELP_COMMANDS {
            tbl.add_row(row![cmd, arg, desc]);
        }

        let fmt = self.format.get_table_format();
        tbl.set_format(fmt);

        let mut command = Cell::new("command");
        command.align(Alignment::CENTER);
        let mut args = Cell::new("args");
        args.align(Alignment::CENTER);
        let mut desc = Cell::new("description");
        desc.align(Alignment::CENTER);

        let info_row = Row::new(vec![command, args, desc]);

        self.output
            .write_from_table(&mut tbl, self.format, self.with_header, info_row);
        self.output.flush();
    }
    fn dot_import(_args: &[&str]) {
        todo!("WIP to implement dot_import function")
    }
    fn dot_imposter(_args: &[&str]) {
        todo!("WIP to implement dot_imposter function")
    }
    fn dot_indexes(&mut self, _args: &[&str]) {
        let sql =
            "SELECT name FROM sqlite_schema WHERE type = 'index' AND name NOT LIKE 'sqlite_%'";
        let mut stmt = self
            .db_conn
            .prepare(sql)
            .expect("unable to create prepared statement");

        let col_count = stmt.column_count();
        let row_title = util::query_colname_as_tbl_rows(&stmt, col_count);
        let row_data =
            util::query_data_as_tbl_rows(&mut stmt, col_count, false, self.null_value.as_deref());

        let fmt = self.format.get_table_format();
        let mut tbl = Table::init(row_data);
        tbl.set_format(fmt);

        self.output
            .write_from_table(&mut tbl, self.format, self.with_header, row_title);
        self.output.flush();
    }
    fn dot_intck(_args: &[&str]) {
        todo!("WIP to implement dot_intck function")
    }
    fn dot_limit(_args: &[&str]) {
        todo!("WIP to implement dot_limit function")
    }
    fn dot_lint(_args: &[&str]) {
        todo!("WIP to implement dot_lint function")
    }
    fn dot_load(_args: &[&str]) {
        todo!("WIP to implement dot_load function")
    }
    fn dot_log(_args: &[&str]) {
        todo!("WIP to implement dot_log function")
    }
    fn dot_mode(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!("current output mode: {:?}", self.format)
        }

        let mode = args[0];
        self.format = TableMode::from(mode);
    }
    fn dot_nonce(_args: &[&str]) {
        todo!("WIP to implement dot_nonce function")
    }
    fn dot_nullvalue(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".nullvalue needs an argument");
            return;
        }
        self.null_value = Some(args[0].to_string());
    }
    fn dot_once(_args: &[&str]) {
        todo!("WIP to implement dot_once function")
    }
    fn dot_open(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".open needs at least one argument");
            return;
        }

        let path = Path::new(args[0]);
        self.cwd.push(path);

        let new_conn = Connection::open(&self.cwd).expect(
            "unable to connect to db or file path can't be converted into C-compatible strings",
        );
        self.db_conn = new_conn;
        self.conn_type = ConnectionType::Persistent(args[0].to_string());
        self.cwd.pop();
    }
    fn dot_output(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".output needs at least one argument");
            return;
        }
        let new_dump_file =
            File::open(args[0]).unwrap_or(File::create(args[0]).expect("unable to create file"));
        self.output = Output::BufferedFile(BufWriter::new(new_dump_file));
    }
    fn dot_parameter(_args: &[&str]) {
        todo!("WIP to implement dot_parameter function")
    }
    fn dot_print(&mut self, args: &[&str]) {
        match &mut self.output {
            Output::BufferedStdout(_) => {
                for arg in args {
                    print!("{}", arg);
                }
            }
            Output::BufferedFile(file) => {
                for arg in args {
                    let _ = writeln!(file, "{}", arg);
                }
            }
        }
    }
    fn dot_progress(_args: &[&str]) {
        todo!("WIP to implement dot_progress function")
    }

    fn dot_prompt(_args: &[&str]) {
        todo!("WIP to implement dot_prompt function")
    }

    fn dot_quit() {
        exit(0)
    }

    fn dot_read(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".read command needs an argument");
            return;
        }

        let path = Path::new(args[0]);
        self.cwd.push(path);

        let db_file = File::open(&self.cwd).expect("unable to open file");
        let reader = BufReader::new(db_file);
        reader.split(b';').for_each(|sql| {
            let sql = sql.expect("unable to unwrap into vector of u8s");
            let sql_str = String::from_utf8(sql).expect("detected a non-utf8 compatible character");
            let trimmed = sql_str.trim();
            if !trimmed.is_empty() {
                self.execute_user_query(trimmed);
                self.cwd.pop();
            }
        });
    }
    fn dot_recover(_args: &[&str]) {
        todo!("WIP to implement dot_recover function")
    }
    fn dot_restore(_args: &[&str]) {
        todo!("WIP to implement dot_restore function")
    }
    fn dot_save(&self, args: &[&str]) {
        if args.is_empty() {
            println!(".save needs an argument");
            return;
        }

        if args[0].is_empty() {
            eprintln!(".save needs a valid file path");
            return;
        }

        let sql = format!("VACUUM INTO '{}'", args[0]);
        let _ = self.db_conn.execute(&sql, []);
        println!("successfully backup/save database!")
    }
    fn dot_scanstats(_args: &[&str]) {
        todo!("WIP to implement dot_scanstats function")
    }

    fn dot_schema(&mut self, _args: &[&str]) {
        let sql = "SELECT sql FROM sqlite_schema WHERE name NOT LIKE '%_autoindex_%' ORDER BY tbl_name, type DESC, name";
        let mut stmt = self
            .db_conn
            .prepare(sql)
            .expect("unable to create prepared statement");

        let col_count = stmt.column_count();
        let row_title = util::query_colname_as_tbl_rows(&stmt, col_count);
        let row_data =
            util::query_data_as_tbl_rows(&mut stmt, col_count, false, self.null_value.as_deref());

        let fmt = self.format.get_table_format();
        let mut tbl = Table::init(row_data);
        tbl.set_format(fmt);

        self.output
            .write_from_table(&mut tbl, self.format, self.with_header, row_title);
        self.output.flush();
    }
    fn dot_separator(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".separator needs at least one argument");
            return;
        }
    }
    fn dot_session(_args: &[&str]) {
        todo!("WIP to implement dot_session function")
    }
    fn dot_sha3sum(_args: &[&str]) {
        todo!("WIP to implement dot_sha3sum function")
    }
    fn dot_shell(&mut self, args: &[&str]) {
        self.dot_system(args);
    }
    fn dot_show(&mut self) {
        let nullval = self.null_value.as_ref();
        let mut tbl = table! {
            ["echo", util::bool_to_onoff(self.with_echo) ],
            ["headers", util::bool_to_onoff(self.with_header) ],
            ["mode", self.format.get_to_str() ],
            ["nullvalue", nullval.unwrap_or(&"".to_string()) ],
            ["output", self.output.get_to_str() ],
            ["filename", self.conn_type.get_to_str()]
        };
        let fmt = self.format.get_table_format();
        tbl.set_format(fmt);

        self.output.write_from_table(
            &mut tbl,
            self.format,
            self.with_header,
            row!["settings", "values"],
        );
        self.output.flush();
    }
    fn dot_stats(_args: &[&str]) {
        todo!("WIP to implement dot_stats function")
    }
    fn dot_system(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".system needs at least one argument");
        }
        let output = Command::new(args[0])
            .args(&args[1..])
            .output()
            .expect("unable to execute command");

        match &mut self.output {
            Output::BufferedStdout(_) => {
                println!(
                    "{:?}",
                    str::from_utf8(&output.stdout).expect("found non utf-8")
                );
            }
            Output::BufferedFile(file) => {
                let out = str::from_utf8(&output.stdout).expect("found non utf-8");
                let _ = writeln!(file, "{}", out);
            }
        }
    }
    fn dot_tables(&mut self) {
        let sql = "SELECT name FROM sqlite_schema
                WHERE type IN ('table','view') AND name NOT LIKE 'sqlite_%'
                ORDER BY 1";
        let mut stmt = self
            .db_conn
            .prepare(sql)
            .expect("unable to create prepared statement");

        let col_count = stmt.column_count();
        let mut row_title = util::query_colname_as_tbl_rows(&stmt, col_count);

        // attempt to rename from `name` to `tables`
        let rename = Cell::new("tables");
        let _ = row_title.set_cell(rename, 0);

        let row_data =
            util::query_data_as_tbl_rows(&mut stmt, col_count, false, self.null_value.as_deref());

        let mut tbl = Table::init(row_data);

        let fmt = self.format.get_table_format();
        tbl.set_format(fmt);

        self.output
            .write_from_table(&mut tbl, self.format, self.with_header, row_title);
        self.output.flush();
    }
    fn dot_timeout(_args: &[&str]) {
        todo!("WIP to implement dot_timeout function")
    }
    fn dot_timer(_args: &[&str]) {
        todo!("WIP to implement dot_timer function")
    }
    fn dot_trace(_args: &[&str]) {
        todo!("WIP to implement dot_trace function")
    }
    fn dot_unmodule(_args: &[&str]) {
        todo!("WIP to implement dot_unmodule function")
    }
    fn dot_version(&mut self) {
        let version: &str = SQLITE_VERSION
            .to_str()
            .expect("version string has a non-utf 8 character");

        let mut sqlite_source_id = SQLITE_SOURCE_ID
            .to_str()
            .expect("version string has a non-utf 8 character")
            .split(" ");

        let date = sqlite_source_id.next().expect("unable to get date");
        let timestamp = sqlite_source_id.next().expect("unable to get timestamp");
        let hash = sqlite_source_id.next().expect("unable to get hash");

        let mut tbl = table!([version, date, timestamp, hash]);
        let fmt = self.format.get_table_format();
        tbl.set_format(fmt);

        let mut version = Cell::new("version");
        let mut date = Cell::new("date");
        let mut timestamp = Cell::new("timestamp");
        let mut hash = Cell::new("hash");

        version.align(Alignment::CENTER);
        date.align(Alignment::CENTER);
        timestamp.align(Alignment::CENTER);
        hash.align(Alignment::CENTER);

        let version_info = Row::new(vec![version, date, timestamp, hash]);

        self.output
            .write_from_table(&mut tbl, self.format, self.with_header, version_info);
        self.output.flush();
    }

    fn dot_vfsinfo(_args: &[&str]) {
        todo!("WIP to implement dot_vfsinfo function")
    }
    fn dot_vfslist(_args: &[&str]) {
        todo!("WIP to implement dot_vfslist function")
    }
    fn dot_vfsname(_args: &[&str]) {
        todo!("WIP to implement dot_vfsname function")
    }
    fn dot_width(_args: &[&str]) {
        todo!("WIP to implement dot_width function")
    }
    fn dot_www(_args: &[&str]) {
        todo!("WIP to implement dot_www function")
    }
}
