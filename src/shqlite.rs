use crate::consts::HELP_COMMANDS;
use prettytable::format::Alignment;
use prettytable::format::TableFormat;
use prettytable::{Cell, Row, Table, row, table};
use rusqlite::Connection;
use rusqlite::ffi::{SQLITE_SOURCE_ID, SQLITE_VERSION};
use rusqlite::types::ValueRef;
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
    pub fn flush(&mut self) {
        match self {
            Output::BufferedStdout(out) => out.flush().expect("unable to flush"),
            Output::BufferedFile(file) => file.flush().expect("unable to flush"),
        }
    }
}

#[derive(Debug, Default)]
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
            _ => {
                println!("unsupported format!");
                *prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE
            }
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

pub struct Shqlite {
    db_conn: Connection,
    output: Output,
    format: TableMode,
    command: Option<String>,
    cwd: PathBuf,
    with_header: bool,
    with_echo: bool,
}

impl Default for Shqlite {
    fn default() -> Self {
        Self {
            db_conn: Connection::open_in_memory()
                .expect("could not establish a temporary database connection"),
            output: Output::BufferedStdout(BufWriter::new(stdout())),
            format: TableMode::Boxed,
            command: None,
            cwd: std::env::current_dir().expect("cwd may not exists or insuffiecient permission"),
            with_header: false,
            with_echo: false,
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

        let mut columns: Vec<Cell> = Vec::with_capacity(col_count);

        for col_idx in 0..col_count {
            let colname = stmt
                .column_name(col_idx)
                .expect("unable to get column named because of an invalid column index");
            let mut cell = Cell::new(colname);
            cell.align(Alignment::CENTER);
            columns.push(cell);
        }

        let mapped_rows = stmt
            .query_map([], |row| {
                let mut cells: Vec<Cell> = Vec::new();
                for col_idx in 0..col_count {
                    let value = row.get_ref(col_idx)?;
                    match value {
                        ValueRef::Null => {
                            let cell = Cell::new("NULL");
                            cells.push(cell);
                        }
                        ValueRef::Text(content) => {
                            let cell = Cell::new(
                                str::from_utf8(content).expect("encountered a non utf-8 character"),
                            );
                            cells.push(cell);
                        }
                        ValueRef::Integer(i) => {
                            let cell = Cell::new(&i.to_string());
                            cells.push(cell);
                        }
                        ValueRef::Real(f) => {
                            let cell = Cell::new(&f.to_string());
                            cells.push(cell);
                        }
                        ValueRef::Blob(b) => {
                            let cell = Cell::new(&format!("<BLOB {} bytes>", b.len()));
                            cells.push(cell);
                        }
                    }
                }
                Ok(cells)
            })
            .expect("unable to query prepared statement");

        let mut table = Table::new();

        let fmt = self.format.get_table_format();
        table.set_format(fmt);

        match self.format {
            TableMode::List | TableMode::Csv | TableMode::Tabs => {
                if self.with_header {
                    table.set_titles(Row::new(columns));
                } else {
                    table.unset_titles();
                }
            }
            _ => table.set_titles(Row::new(columns)),
        }
        // the rest contains the value of that column
        for row_result in mapped_rows.flatten() {
            table.add_row(Row::new(row_result));
        }

        match &mut self.output {
            Output::BufferedStdout(_) => {
                table.printstd();
            }
            Output::BufferedFile(file) => {
                table.print(file).expect("unable to write to file");
            }
        }

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
            ".dbconfig" => Self::dot_dbconfig(),
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
            ".nullvalue" => Self::dot_nullvalue(dot_cmd_args),
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
            ".separator" => Self::dot_separator(dot_cmd_args),
            ".session" => Self::dot_session(dot_cmd_args),
            ".sha3sum" => Self::dot_sha3sum(dot_cmd_args),
            ".shell" => self.dot_shell(dot_cmd_args),
            ".show" => Self::dot_show(dot_cmd_args),
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
        let mut stmt = self
            .db_conn
            .prepare("SELECT seq , name , file FROM pragma_database_list")
            .expect("unable to create prepared statement");

        let db_infos = stmt
            .query_map([], |row| {
                let mut db_info_row: Vec<Cell> = Vec::with_capacity(3);
                let seq = row.get::<_, u32>(0).expect("can't get seq columns");
                db_info_row.push(Cell::new(&seq.to_string()));
                let name = row.get::<_, String>(1).expect("can't get name columns");
                db_info_row.push(Cell::new(&name));
                let file = row.get::<_, String>(2).expect("can't get file columns");
                db_info_row.push(Cell::new(&file));
                Ok(db_info_row)
            })
            .expect("could not query .databases");

        let mut table = Table::new();

        let fmt = self.format.get_table_format();
        table.set_format(fmt);

        let mut seq = Cell::new("seq");
        seq.align(Alignment::CENTER);
        let mut name = Cell::new("name");
        name.align(Alignment::CENTER);
        let mut file = Cell::new("file");
        file.align(Alignment::CENTER);

        let title = Row::new(vec![seq, name, file]);

        match self.format {
            TableMode::List | TableMode::Csv | TableMode::Tabs => {
                if self.with_header {
                    table.set_titles(title);
                } else {
                    table.unset_titles();
                }
            }
            _ => table.set_titles(title),
        }

        for db_info in db_infos.flatten() {
            table.add_row(Row::new(db_info));
        }

        match &mut self.output {
            Output::BufferedStdout(_) => {
                table.printstd();
            }
            Output::BufferedFile(file) => {
                table.print(file).expect("unable to write to file");
            }
        }
    }
    fn dot_dbconfig() {}
    fn dot_dbinfo(_args: &[&str]) {
        todo!("WIP to implement dot_dbinfo function")
    }
    fn dot_dbtotxt(_args: &[&str]) {
        todo!("WIP to implement dot_dbtotxt function")
    }
    fn dot_dump(&mut self, _args: &[&str]) {
        match &mut self.output {
            Output::BufferedStdout(out) => {
                let _ = writeln!(out, "{}", "PRAGMA foreign_keys=OFF;\nBEGIN TRANSACTION;");
            }
            Output::BufferedFile(file) => {
                let _ = writeln!(file, "{}", "PRAGMA foreign_keys=OFF;\nBEGIN TRANSACTION;");
            }
        }

        let get_table_names =
            "SELECT name, type, sql FROM sqlite_schema WHERE  name NOT LIKE 'sqlite_%' ORDER BY 1";

        let mut stmt = self
            .db_conn
            .prepare(get_table_names)
            .expect("unable to construct a prepared statement");

        let table_names = stmt
            .query_map([], |row| {
                let name = row.get::<_, String>(0).expect("can't get name columns");
                let typenme = row.get::<_, String>(1).expect("can't get type columns");
                let sql = row.get::<_, String>(2).expect("can't get sql columns");
                Ok((name, typenme, sql))
            })
            .expect("could not query .tables");

        let table_names = table_names.flatten();

        for (name, typnme, sql) in table_names {
            if typnme == "table" || typnme == "view" {
                let values_sql = format!("SELECT * FROM {}", name);
                let mut values_stmt = self
                    .db_conn
                    .prepare(&values_sql)
                    .expect("unable to construct a prepared statement");

                let columns_count = values_stmt.column_count();

                let insert_into_values = values_stmt
                    .query_map([], |row| {
                        let mut cells: Vec<String> = Vec::new();
                        for col_idx in 0..columns_count {
                            let value = row.get_ref(col_idx)?;
                            match value {
                                ValueRef::Null => {
                                    cells.push("NULL".to_string());
                                }
                                ValueRef::Text(content) => {
                                    let utf_str = str::from_utf8(content)
                                        .expect("encountered some non utf-8 string");
                                    cells.push(format!("'{}'", utf_str.to_string()));
                                }
                                ValueRef::Integer(i) => {
                                    cells.push(i.to_string());
                                }
                                ValueRef::Real(f) => {
                                    cells.push(f.to_string());
                                }
                                ValueRef::Blob(b) => {
                                    cells.push(format!("<BLOB {} bytes>", b.len()));
                                }
                            }
                        }
                        Ok(cells)
                    })
                    .expect("could not query values");

                match &mut self.output {
                    Output::BufferedStdout(out) => {
                        let _ = writeln!(out, "{};", sql);
                    }
                    Output::BufferedFile(file) => {
                        let _ = writeln!(file, "{};", sql);
                    }
                }

                for insert_into in insert_into_values.flatten() {
                    let joined = insert_into.join(", ");
                    match &mut self.output {
                        Output::BufferedStdout(out) => {
                            let _ = writeln!(out, "INSERT INTO {} VALUES ({});", name, joined);
                        }
                        Output::BufferedFile(file) => {
                            let _ = writeln!(file, "INSERT INTO {} VALUES ({});", name, joined);
                        }
                    }
                }
            } else if typnme == "index" {
                match &mut self.output {
                    Output::BufferedStdout(out) => {
                        let _ = writeln!(out, "{};", sql);
                    }
                    Output::BufferedFile(file) => {
                        let _ = writeln!(file, "{};", sql);
                    }
                }
            }
        }
        match &mut self.output {
            Output::BufferedStdout(out) => {
                let _ = writeln!(out, "{}", "COMMIT;");
            }
            Output::BufferedFile(file) => {
                let _ = writeln!(file, "{}", "COMMIT;");
            }
        }

        self.output.flush();
    }

    fn dot_echo(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".echo needs an argument...");
            return;
        }

        let arg = args[0];
        match arg {
            "on" => self.with_echo = true,
            "off" => self.with_echo = false,
            _ => {
                println!("unrecognized argument.");
                return;
            }
        }
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
        match arg {
            "yes" => self.with_header = true,
            "no" => self.with_header = false,
            _ => {
                println!("ERROR: Not a boolean value: \"{}\".", arg);
            }
        }
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

        match self.format {
            TableMode::List | TableMode::Csv | TableMode::Tabs => {
                if self.with_header {
                    tbl.set_titles(info_row);
                } else {
                    tbl.unset_titles();
                }
            }
            _ => tbl.set_titles(info_row),
        }

        match &mut self.output {
            Output::BufferedStdout(_) => {
                tbl.printstd();
            }
            Output::BufferedFile(file) => {
                tbl.print(file).expect("unable to write to file");
            }
        }
    }
    fn dot_import(_args: &[&str]) {
        todo!("WIP to implement dot_import function")
    }
    fn dot_imposter(_args: &[&str]) {
        todo!("WIP to implement dot_imposter function")
    }
    fn dot_indexes(&mut self, _args: &[&str]) {
        let mut stmt = self
            .db_conn
            .prepare(
                "SELECT name FROM sqlite_schema WHERE type = 'index' AND name NOT LIKE 'sqlite_%'",
            )
            .expect("unable to create prepared statement");

        let indexes_rows = stmt
            .query_map([], |row| {
                let name = row.get::<_, String>(0).expect("can't get name columns");
                Ok(name)
            })
            .expect("could not query .indexes");

        let mut table = Table::new();
        let fmt = self.format.get_table_format();
        table.set_format(fmt);

        let mut indexes_cell = Cell::new("indexes");
        indexes_cell.align(Alignment::CENTER);
        let index_row = Row::new(vec![indexes_cell]);

        match self.format {
            TableMode::List | TableMode::Csv | TableMode::Tabs => {
                if self.with_header {
                    table.set_titles(index_row);
                } else {
                    table.unset_titles();
                }
            }
            _ => table.set_titles(index_row),
        }

        for index_row in indexes_rows.flatten() {
            table.add_row(Row::new(vec![Cell::new(&index_row)]));
        }

        match &mut self.output {
            Output::BufferedStdout(_) => {
                table.printstd();
            }
            Output::BufferedFile(file) => {
                table.print(file).expect("unable to write to file");
            }
        }
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
    fn dot_nullvalue(_args: &[&str]) {
        todo!("WIP to implement dot_nullvalue function")
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
        let mut stmt = self
            .db_conn
            .prepare("SELECT sql FROM sqlite_schema WHERE name NOT LIKE '%_autoindex_%' ORDER BY tbl_name, type DESC, name")
            .expect("unable to create prepared statement");

        let schemas = stmt
            .query_map([], |row| {
                let schema = row.get::<_, String>(0).expect("can't get name columns");
                Ok(schema)
            })
            .expect("could not run query .schema");

        let mut table = Table::new();
        let fmt = self.format.get_table_format();
        table.set_format(fmt);

        let mut title = Cell::new("schema");
        title.align(Alignment::CENTER);
        let row_title = Row::new(vec![title]);

        match self.format {
            TableMode::List | TableMode::Csv | TableMode::Tabs => {
                if self.with_header {
                    table.set_titles(row_title)
                } else {
                    table.unset_titles();
                }
            }
            _ => table.set_titles(row_title),
        }

        for schema in schemas.flatten() {
            table.add_row(Row::new(vec![Cell::new(&schema)]));
        }

        match &mut self.output {
            Output::BufferedStdout(_) => table.printstd(),
            Output::BufferedFile(file) => {
                table.print(file).expect("unable to write to file");
            }
        }
    }
    fn dot_separator(_args: &[&str]) {
        todo!("WIP to implement dot_separator function")
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
    fn dot_show(_args: &[&str]) {
        todo!("WIP to implement dot_show function")
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
        let mut stmt = self
            .db_conn
            .prepare(
                "SELECT name FROM sqlite_schema
        WHERE type IN ('table','view') AND name NOT LIKE 'sqlite_%'
        ORDER BY 1",
            )
            .expect("unable to create prepared statement");

        let table_name_rows = stmt
            .query_map([], |row| {
                let name = row.get::<_, String>(0).expect("can't get name columns");
                Ok(name)
            })
            .expect("could not query .tables");

        let mut table = Table::new();
        let fmt = self.format.get_table_format();
        table.set_format(fmt);

        let mut tables_cell = Cell::new("tables");
        tables_cell.align(Alignment::CENTER);
        let table_row = Row::new(vec![tables_cell]);

        match self.format {
            TableMode::List | TableMode::Csv | TableMode::Tabs => {
                if self.with_header {
                    table.set_titles(table_row);
                } else {
                    table.unset_titles();
                }
            }
            _ => table.set_titles(table_row),
        }

        for row_name_result in table_name_rows.flatten() {
            table.add_row(Row::new(vec![Cell::new(&row_name_result)]));
        }

        match &mut self.output {
            Output::BufferedStdout(_) => table.printstd(),
            Output::BufferedFile(file) => {
                table.print(file).expect("unable to write to file");
            }
        }
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

        let mut table = table!([version, date, timestamp, hash]);
        let fmt = self.format.get_table_format();
        table.set_format(fmt);

        let mut version = Cell::new("version");
        let mut date = Cell::new("date");
        let mut timestamp = Cell::new("timestamp");
        let mut hash = Cell::new("hash");

        version.align(Alignment::CENTER);
        date.align(Alignment::CENTER);
        timestamp.align(Alignment::CENTER);
        hash.align(Alignment::CENTER);

        let version_info = Row::new(vec![version, date, timestamp, hash]);

        match self.format {
            TableMode::List | TableMode::Csv | TableMode::Tabs => table.unset_titles(),
            _ => {
                if self.with_header {
                    table.set_titles(version_info);
                } else {
                    table.unset_titles();
                }
            }
        }

        match &mut self.output {
            Output::BufferedStdout(_) => table.printstd(),
            Output::BufferedFile(file) => {
                table.print(file).expect("unable to write to file");
            }
        }
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
