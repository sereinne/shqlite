use clap::Parser;
use prettytable::format::Alignment;
use prettytable::format::{FormatBuilder, LinePosition, LineSeparator, TableFormat};
use prettytable::{Cell, Row, Table, table};
use rusqlite::Connection;
use rusqlite::ffi::{SQLITE_SOURCE_ID, SQLITE_VERSION};
use rusqlite::types::ValueRef;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::process::{Command, exit};
use std::vec;

#[derive(Default)]
pub enum Output {
    #[default]
    StandardOut,
    DumpFile(File),
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
            TableMode::Tabs => FormatBuilder::new()
                .column_separator('\t')
                .padding(0, 0)
                .indent(0)
                .build(),
            TableMode::Csv => FormatBuilder::new()
                .column_separator(',')
                .padding(0, 0)
                .indent(0)
                .build(),
            TableMode::Column => FormatBuilder::new()
                .column_separator(' ')
                .separator(LinePosition::Title, LineSeparator::new('-', ' ', ' ', ' '))
                .padding(0, 0)
                .build(),
            TableMode::Markdown => FormatBuilder::new()
                .column_separator('|')
                .borders('|')
                .separator(LinePosition::Title, LineSeparator::new('-', '|', '|', '|'))
                .padding(1, 1)
                .build(),
            TableMode::List => FormatBuilder::new()
                .column_separator('|')
                .padding(0, 0)
                .indent(0)
                .build(),
            TableMode::Boxed => FormatBuilder::new()
                .column_separator('│')
                .separator(LinePosition::Title, LineSeparator::new('─', '┼', '├', '┤'))
                .padding(1, 1)
                .separator(LinePosition::Bottom, LineSeparator::new('─', '┴', '╰', '╯'))
                .separator(LinePosition::Top, LineSeparator::new('─', '┬', '╭', '╮'))
                .borders('│')
                .build(),
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

#[derive(Parser, Debug)]
#[command(name = "shqlite")]
#[command(about = "A simple terminal `sqlite` client in Rust", long_about = None)]
pub struct App {
    /// path to database file
    #[arg(short, long, default_value = "")]
    path: String,

    /// table format (.mode)
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
    mode: String,

    /// where to output query results
    #[arg(short, long, default_value = "")]
    output: String,

    /// sql or dot command to execute before going into the shell
    #[arg(short, long)]
    query: Option<String>,
}

// MSRV without prettytable: 1.88
pub struct Shqlite {
    db_conn: Connection,
    output: Output,
    format: TableMode,
}

impl Default for Shqlite {
    fn default() -> Self {
        Self {
            db_conn: Connection::open_in_memory()
                .expect("could not establish a temporary database connection"),
            output: Output::StandardOut,
            format: TableMode::Boxed,
        }
    }
}

impl Shqlite {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_conn(mut self, path: &str) -> Self {
        if !path.is_empty() {
            self.db_conn = Connection::open(path).expect("unable to open db file");
            return self;
        }
        self
    }

    pub fn set_format(mut self, fmt: &str) -> Self {
        if !fmt.is_empty() {
            self.format = TableMode::from(fmt);
            return self;
        }
        self
    }

    pub fn set_output(mut self, out: &str) -> Self {
        if !out.is_empty() {
            let path = Path::new(out);

            if !path.exists() {
                let file = File::create_new(path).expect("unable to create file");
                self.output = Output::DumpFile(file);
                return self;
            }

            let file = File::open(path).expect("unable to open file");
            self.output = Output::DumpFile(file);
            return self;
        }
        self
    }

    pub fn build(self) -> Self {
        Self {
            db_conn: self.db_conn,
            format: self.format,
            output: self.output,
        }
    }
    pub fn start_repl(&mut self) -> rustyline::Result<()> {
        let mut editor = DefaultEditor::new()?;
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

            println!("Successfully executed command!");
            return;
        }

        let mut columns: Vec<Cell> = Vec::with_capacity(col_count);

        for col_idx in 0..col_count {
            let colname = stmt
                .column_name(col_idx)
                .expect("out of bound index of column");
            let mut cell = Cell::new(colname);
            cell.align(Alignment::CENTER);
            columns.push(cell);
        }

        let mapped_rows = stmt
            .query_map([], |row| {
                let mut cells: Vec<Cell> = Vec::new();
                for col_idx in 0..col_count {
                    let value = row
                        .get_ref(col_idx)
                        .expect("unable to get a reference to some row value");
                    match value {
                        ValueRef::Null => cells.push(Cell::new("NULL")),
                        ValueRef::Text(content) => {
                            cells.push(Cell::new(str::from_utf8(content).expect("non-utf9")))
                        }
                        ValueRef::Integer(i) => cells.push(Cell::new(&i.to_string())),
                        ValueRef::Real(f) => cells.push(Cell::new(&f.to_string())),
                        ValueRef::Blob(b) => {
                            cells.push(Cell::new(&format!("<BLOB {} bytes>", b.len())))
                        }
                    }
                }
                Ok(cells)
            })
            .expect("unable to query with a prepared statement");

        let mut table = Table::new();

        let fmt = self.format.get_table_format();
        table.set_format(fmt);

        match self.format {
            TableMode::List | TableMode::Csv | TableMode::Tabs => table.unset_titles(),
            _ => table.set_titles(Row::new(columns)),
        }
        // the rest contains the value of that column
        for row_result in mapped_rows.flatten() {
            table.add_row(Row::new(row_result));
        }

        match &mut self.output {
            Output::StandardOut => table.printstd(),
            Output::DumpFile(file) => {
                table.print(file).expect("unable to write to file");
            }
        }
    }

    fn execute_dot_command(&mut self, user_input: &str) {
        let dot_cmd_with_args = user_input.split(" ").collect::<Vec<_>>();
        let dot_cmd: &str = dot_cmd_with_args[0];
        let dot_cmd_args = &dot_cmd_with_args[1..];

        match dot_cmd {
            ".archive" => Self::dot_archive(dot_cmd_args),
            ".auth" => Self::dot_auth(dot_cmd_args),
            ".backup" => Self::dot_backup(dot_cmd_args),
            ".bail" => Self::dot_bail(dot_cmd_args),
            ".cd" => Self::dot_cd(dot_cmd_args),
            ".changes" => Self::dot_changes(dot_cmd_args),
            ".check" => Self::dot_check(dot_cmd_args),
            ".clone" => Self::dot_clone(dot_cmd_args),
            ".connection" => Self::dot_connection(dot_cmd_args),
            ".crlf" => Self::dot_crlf(dot_cmd_args),
            ".databases" => self.dot_databases(dot_cmd_args),
            ".dbconfig" => Self::dot_dbconfig(),
            ".dbinfo" => Self::dot_dbinfo(dot_cmd_args),
            ".dbtotxt" => Self::dot_dbtotxt(dot_cmd_args),
            ".dump" => Self::dot_dump(dot_cmd_args),
            ".echo" => self.dot_echo(dot_cmd_args),
            ".eqp" => Self::dot_eqp(dot_cmd_args),
            ".excel" => Self::dot_excel(dot_cmd_args),
            ".exit" => Self::dot_exit(dot_cmd_args),
            ".expert" => Self::dot_expert(dot_cmd_args),
            ".explain" => Self::dot_explain(dot_cmd_args),
            ".filectrl" => Self::dot_filectrl(dot_cmd_args),
            ".fullschema" => Self::dot_fullschema(dot_cmd_args),
            ".headers" => Self::dot_headers(dot_cmd_args),
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
            ".save" => Self::dot_save(dot_cmd_args),
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
    fn dot_backup(_args: &[&str]) {
        todo!("WIP to implement dot_backup function")
    }
    fn dot_bail(_args: &[&str]) {
        todo!("WIP to implement dot_bail function")
    }
    fn dot_cd(args: &[&str]) {
        if args.is_empty() {
            println!(".cd needs an argument to run");
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
            TableMode::List | TableMode::Csv | TableMode::Tabs => table.unset_titles(),
            _ => table.set_titles(title),
        }

        for db_info in db_infos.flatten() {
            table.add_row(Row::new(db_info));
        }

        match &mut self.output {
            Output::StandardOut => table.printstd(),
            Output::DumpFile(file) => {
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
    fn dot_dump(_args: &[&str]) {
        todo!("WIP to implement dot_dump function")
    }
    fn dot_echo(&mut self, args: &[&str]) {
        self.dot_print(args);
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
    fn dot_headers(_args: &[&str]) {
        todo!("WIP to implement dot_headers function")
    }
    fn dot_help(&mut self) {
        let mut table = table!(
            [".archive", "...", "Manage SQL archives"],
            [".auth", "ON|OFF", "Show authorizer callbacks"],
            [
                ".backup",
                "?DB? FILE",
                "Backup DB (default \"main\") to FILE"
            ],
            [
                ".bail",
                "on|off",
                "Stop after hitting an error. Default OFF"
            ],
            [
                ".cd",
                "DIRECTORY",
                "Change the working directory to DIRECTORY"
            ],
            [".changes", "on|off", "Show number of rows changed by SQL"],
            [
                ".check",
                "GLOB",
                "Fail if output since .testcase does not match"
            ],
            [
                ".clone",
                "NEWDB",
                "Clone data into NEWDB from the existing database"
            ],
            [
                ".connection",
                "[close] [#]",
                "Open or close an auxiliary database connection"
            ],
            [
                ".crlf",
                "?on|off?",
                "Whether or not to use \\r\\n line endings"
            ],
            [
                ".databases",
                "",
                "List names and files of attached databases"
            ],
            [
                ".dbconfig",
                "?op? ?val?",
                "List or change sqlite3_db_config() options"
            ],
            [
                ".dbinfo",
                "?DB?",
                "Show status information about the database"
            ],
            [".dbtotxt", "", "Hex dump of the database file"],
            [".dump", "?OBJECTS?", "Render database content as SQL"],
            [".echo", "on|off", "Turn command echo on or off"],
            [
                ".eqp",
                "on|off|full|...",
                "Enable or disable automatic EXPLAIN QUERY PLAN"
            ],
            [
                ".excel",
                "",
                "Display the output of next command in spreadsheet"
            ],
            [".exit", "?CODE?", "Exit this program with return-code CODE"],
            [".expert", "", "EXPERIMENTAL. Suggest indexes for queries"],
            [
                ".explain",
                "?on|off|auto?",
                "Change the EXPLAIN formatting mode. Default: auto"
            ],
            [
                ".filectrl",
                "CMD ...",
                "Run various sqlite3_file_control() operations"
            ],
            [
                ".fullschema",
                "?--indent?",
                "Show schema and the content of sqlite_stat tables"
            ],
            [".headers", "on|off", "Turn display of headers on or off"],
            [".help", "?-all? ?PATTERN?", "Show help text for PATTERN"],
            [".import", "FILE TABLE", "Import data from FILE into TABLE"],
            [
                ".imposter",
                "INDEX TABLE",
                "Create imposter table TABLE on index INDEX"
            ],
            [".indexes", "?TABLE?", "Show names of indexes"],
            [
                ".intck",
                "?STEPS_PER_UNLOCK?",
                "Run an incremental integrity check on the db"
            ],
            [
                ".limit",
                "?LIMIT? ?VAL?",
                "Display or change the value of an SQLITE_LIMIT"
            ],
            [".lint", "OPTIONS", "Report potential schema issues."],
            [".load", "FILE ?ENTRY?", "Load an extension library"],
            [
                ".log",
                "FILE|on|off",
                "Turn logging on or off. FILE can be stderr/stdout"
            ],
            [".mode", "?MODE? ?OPTIONS?", "Set output mode"],
            [
                ".nonce",
                "STRING",
                "Suspend safe mode for one command if nonce matches"
            ],
            [".nullvalue", "STRING", "Use STRING in place of NULL values"],
            [
                ".once",
                "?OPTIONS? ?FILE?",
                "Output for the next SQL command only to FILE"
            ],
            [
                ".open",
                "?OPTIONS? ?FILE?",
                "Close existing database and reopen FILE"
            ],
            [
                ".output",
                "?FILE?",
                "Send output to FILE or stdout if FILE is omitted"
            ],
            [".parameter", "CMD ...", "Manage SQL parameter bindings"],
            [".print", "STRING...", "Print literal STRING"],
            [
                ".progress",
                "N",
                "Invoke progress handler after every N opcodes"
            ],
            [".prompt", "MAIN CONTINUE", "Replace the standard prompts"],
            [
                ".quit",
                "",
                "Stop interpreting input stream, exit if primary."
            ],
            [".read", "FILE", "Read input from FILE or command output"],
            [
                ".recover",
                "",
                "Recover as much data as possible from corrupt db."
            ],
            [
                ".restore",
                "?DB? FILE",
                "Restore content of DB (default \"main\") from FILE"
            ],
            [
                ".save",
                "?OPTIONS? FILE",
                "Write database to FILE (an alias for .backup ...)"
            ],
            [
                ".scanstats",
                "on|off|est",
                "Turn sqlite3_stmt_scanstatus() metrics on or off"
            ],
            [
                ".schema",
                "?PATTERN?",
                "Show the CREATE statements matching PATTERN"
            ],
            [
                ".separator",
                "COL ?ROW?",
                "Change the column and row separators"
            ],
            [".session", "?NAME? CMD ...", "Create or control sessions"],
            [".sha3sum", "...", "Compute a SHA3 hash of database content"],
            [".shell", "CMD ARGS...", "Run CMD ARGS... in a system shell"],
            [".show", "", "Show the current values for various settings"],
            [".stats", "?ARG?", "Show stats or turn stats on or off"],
            [
                ".system",
                "CMD ARGS...",
                "Run CMD ARGS... in a system shell"
            ],
            [
                ".tables",
                "?TABLE?",
                "List names of tables matching LIKE pattern TABLE"
            ],
            [
                ".timeout",
                "MS",
                "Try opening locked tables for MS milliseconds"
            ],
            [".timer", "on|off", "Turn SQL timer on or off"],
            [
                ".trace",
                "?OPTIONS?",
                "Output each SQL statement as it is run"
            ],
            [".unmodule", "NAME ...", "Unregister virtual table modules"],
            [".version", "", "Show source, library and compiler versions"],
            [".vfsinfo", "?AUX?", "Information about the top-level VFS"],
            [".vfslist", "", "List all available VFSes"],
            [".vfsname", "?AUX?", "Print the name of the VFS stack"],
            [
                ".width",
                "NUM1 NUM2 ...",
                "Set minimum column widths for columnar output"
            ],
            [
                ".www",
                "",
                "Display output of the next command in web browser"
            ]
        );

        let fmt = self.format.get_table_format();
        table.set_format(fmt);

        let mut command = Cell::new("command");
        command.align(Alignment::CENTER);
        let mut args = Cell::new("args");
        args.align(Alignment::CENTER);
        let mut desc = Cell::new("description");
        desc.align(Alignment::CENTER);

        let info_row = Row::new(vec![command, args, desc]);

        match self.format {
            TableMode::List | TableMode::Csv | TableMode::Tabs => table.unset_titles(),
            _ => table.set_titles(info_row),
        }

        match &mut self.output {
            Output::StandardOut => table.printstd(),
            Output::DumpFile(file) => {
                table.print(file).expect("unable to write to file");
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
            TableMode::List | TableMode::Csv | TableMode::Tabs => table.unset_titles(),
            _ => table.set_titles(index_row),
        }

        for index_row in indexes_rows.flatten() {
            table.add_row(Row::new(vec![Cell::new(&index_row)]));
        }

        match &mut self.output {
            Output::StandardOut => table.printstd(),
            Output::DumpFile(file) => {
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
        let new_conn = Connection::open(args[0]).expect(
            "unable to connect to db or file path can't be converted into C-compatible strings",
        );
        self.db_conn = new_conn;
    }
    fn dot_output(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".output needs at least one argument");
            return;
        }
        let new_dump_file =
            File::open(args[0]).unwrap_or(File::create(args[0]).expect("unable to create file"));
        self.output = Output::DumpFile(new_dump_file);
    }
    fn dot_parameter(_args: &[&str]) {
        todo!("WIP to implement dot_parameter function")
    }
    fn dot_print(&mut self, args: &[&str]) {
        match &mut self.output {
            Output::StandardOut => {
                for arg in args {
                    println!("{}", arg);
                }
            }
            Output::DumpFile(file) => {
                let mut bufwriter = BufWriter::new(file);
                for arg in args {
                    let bytes_writen = bufwriter
                        .write(arg.as_bytes())
                        .expect("can't write to file");
                    println!("written {}", bytes_writen);
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
        let db_file = File::open(args[0]).expect("unable to open file");
        let reader = BufReader::new(db_file);
        reader.split(b';').for_each(|sql| {
            let sql = sql.expect("unable to unwrap into vector of u8s");
            let sql_str = String::from_utf8(sql).expect("detected a non-utf8 compatible character");
            let trimmed = sql_str.trim();
            if !trimmed.is_empty() {
                self.execute_user_query(trimmed);
            }
        });
    }
    fn dot_recover(_args: &[&str]) {
        todo!("WIP to implement dot_recover function")
    }
    fn dot_restore(_args: &[&str]) {
        todo!("WIP to implement dot_restore function")
    }
    fn dot_save(_args: &[&str]) {
        todo!("WIP to implement dot_save function")
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
            TableMode::List | TableMode::Csv | TableMode::Tabs => table.unset_titles(),
            _ => table.set_titles(row_title),
        }

        for schema in schemas.flatten() {
            table.add_row(Row::new(vec![Cell::new(&schema)]));
        }

        match &mut self.output {
            Output::StandardOut => table.printstd(),
            Output::DumpFile(file) => {
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
            Output::StandardOut => {
                println!(
                    "{:?}",
                    str::from_utf8(&output.stdout).expect("found non utf-8")
                );
            }
            Output::DumpFile(file) => {
                let mut bufwriter = BufWriter::new(file);
                let bytes_written = bufwriter
                    .write(&output.stdout)
                    .expect("can't write to files");
                println!("bytes written {}", bytes_written);
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
            TableMode::List | TableMode::Csv | TableMode::Tabs => table.unset_titles(),
            _ => table.set_titles(table_row),
        }

        for row_name_result in table_name_rows.flatten() {
            table.add_row(Row::new(vec![Cell::new(&row_name_result)]));
        }

        match &mut self.output {
            Output::StandardOut => table.printstd(),
            Output::DumpFile(file) => {
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
            _ => table.set_titles(version_info),
        }

        match &mut self.output {
            Output::StandardOut => table.printstd(),
            Output::DumpFile(file) => {
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

fn main() -> anyhow::Result<()> {
    let app = App::parse();

    let mut shqlite = Shqlite::default()
        .set_conn(&app.path)
        .set_format(&app.mode)
        .set_output(&app.output)
        .build();

    shqlite.start_repl()?;

    Ok(())
}
