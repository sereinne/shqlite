use crate::config::{Context, Output, TableMode};
use crate::util;
use prettytable::format::TableFormat;
use prettytable::{Table, row, table};
use rusqlite::Error as RSQE;
use rusqlite::config::DbConfig;
use rusqlite::ffi::{SQLITE_SOURCE_ID, SQLITE_VERSION};
use rusqlite::{Connection, MAIN_DB};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

pub struct CommandRunner<'a> {
    ctx: &'a mut Context,
}

impl<'a> CommandRunner<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx }
    }

    pub fn run_command(&mut self, input: &str) -> rusqlite::Result<()> {
        if self.ctx.with_echo {
            let writer: &mut dyn Write = match &mut self.ctx.output {
                Output::BufferedStdout(out) => out,
                Output::BufferedFile(f) => f,
            };

            let _ = write!(writer, "{}", input);
        }

        if input.starts_with(".") {
            let splitted = input.split(" ").collect::<Vec<&str>>();
            let dot_cmd = splitted[0];
            let dot_cmd_args = &splitted[1..];
            self.run_dot_command(dot_cmd, dot_cmd_args);
        } else {
            self.run_user_query(input)?;
        }

        Ok(())
    }

    fn run_dot_command(&mut self, dot_cmd: &str, args: &[&str]) {
        match dot_cmd {
            ".archive" => self.dot_archive(args),
            ".auth" => self.dot_auth(args),
            ".bail" => self.dot_bail(args),
            ".cd" => self.dot_cd(args),
            ".changes" => self.dot_changes(args),
            ".check" => self.dot_check(args),
            ".clone" => self.dot_clone(args),
            ".connection" => self.dot_connection(args),
            ".crlf" => self.dot_crlf(args),
            ".databases" => self.dot_databases(args).unwrap(),
            ".dbconfig" => self.dot_dbconfig(args),
            ".dbinfo" => self.dot_dbinfo(args),
            ".dbtotxt" => self.dot_dbtotxt(args),
            ".dump" => self.dot_dump(args),
            ".echo" => self.dot_echo(args),
            ".eqp" => self.dot_eqp(args),
            ".excel" => self.dot_excel(args),
            ".expert" => self.dot_expert(args),
            ".explain" => self.dot_explain(args),
            ".filectrl" => self.dot_filectrl(args),
            ".fullschema" => self.dot_fullschema(args),
            ".headers" => self.dot_headers(args),
            ".help" => self.dot_help(args),
            ".import" => self.dot_import(args),
            ".imposter" => self.dot_imposter(args),
            ".indexes" => self.dot_indexes(args).unwrap(),
            ".intck" => self.dot_intck(args),
            ".limit" => self.dot_limit(args),
            ".lint" => self.dot_lint(args),
            ".load" => self.dot_load(args),
            ".log" => self.dot_log(args),
            ".mode" => self.dot_mode(args),
            ".nonce" => self.dot_nonce(args),
            ".nullvalue" => self.dot_nullvalue(args),
            ".once" => self.dot_once(args),
            ".open" => self.dot_open(args),
            ".output" => self.dot_output(args),
            ".parameter" => self.dot_parameter(args),
            ".print" => self.dot_print(args),
            ".progress" => self.dot_progress(args),
            ".prompt" => self.dot_prompt(args),
            ".read" => self.dot_read(args),
            ".recover" => self.dot_recover(args),
            ".restore" => self.dot_restore(args),
            ".save" | ".backup" => self.dot_save(args),
            ".scanstats" => self.dot_scanstats(args),
            ".schema" => self.dot_schema(args).unwrap(),
            ".separator" => self.dot_separator(args),
            ".session" => self.dot_session(args),
            ".sha3sum" => self.dot_sha3sum(args),
            ".show" => self.dot_show(args),
            ".stats" => self.dot_stats(args),
            ".system" | ".shell" => self.dot_system(args),
            ".tables" => self.dot_tables(args).unwrap(),
            ".timeout" => self.dot_timeout(args),
            ".timer" => self.dot_timer(args),
            ".trace" => self.dot_trace(args),
            ".unmodule" => self.dot_unmodule(args),
            ".version" => self.dot_version(args),
            ".vfsinfo" => self.dot_vfsinfo(args),
            ".vfslist" => self.dot_vfslist(args),
            ".vfsname" => self.dot_vfsname(args),
            ".width" => self.dot_width(args),
            ".www" => self.dot_www(args),
            _ => eprintln!(
                "Error: unknown command or invalid arguments:  \"{}\". Enter \".help\" for help",
                dot_cmd
            ),
        }
    }

    fn run_user_query(&mut self, query: &str) -> rusqlite::Result<()> {
        match self.ctx.conn.borrow().prepare(query) {
            Ok(mut stmt) => {
                let col_count = stmt.column_count();

                if col_count == 0 {
                    stmt.execute(())?;
                    return Ok(());
                }

                let column_names = util::query_title_row(&mut stmt, col_count, self.ctx.mode)?;
                let row_datas = util::query_data_rows(
                    &mut stmt,
                    col_count,
                    self.ctx.mode,
                    self.ctx.null_value_repr.as_ref(),
                )?;

                util::construct_and_print_output(
                    &mut self.ctx.output,
                    self.ctx.mode,
                    column_names,
                    row_datas,
                    self.ctx.with_header,
                );
            }
            Err(e) => match e {
                RSQE::SqlInputError { msg, sql, .. } => {
                    eprintln!("ERROR: {} is in invalid sqlite query", sql);
                    eprintln!("{}", msg);
                }
                RSQE::SqliteFailure(_, msg) => {
                    eprintln!(
                        "ERROR: {}",
                        msg.unwrap_or("something bad happended".to_string())
                    );
                }
                _ => eprintln!("ERROR: {:?}", e),
            },
        }

        Ok(())
    }

    fn dot_archive(&mut self, _args: &[&str]) {}
    fn dot_auth(&mut self, _args: &[&str]) {}
    fn dot_bail(&mut self, _args: &[&str]) {}
    fn dot_cd(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".cd needs an argument");
            return;
        }

        let path = PathBuf::from(args[0]);
        if !path.exists() {
            println!("path doesn't exist {}", path.display());
            return;
        }

        if path.is_absolute() {
            self.ctx.cwd = path;
            return;
        } else if path.is_dir() && path.is_relative() {
            self.ctx.cwd.push(path);
        }
    }
    fn dot_changes(&mut self, _args: &[&str]) {}
    fn dot_check(&mut self, _args: &[&str]) {}
    fn dot_clone(&mut self, _args: &[&str]) {}
    fn dot_connection(&mut self, _args: &[&str]) {}
    fn dot_crlf(&mut self, _args: &[&str]) {}
    fn dot_databases(&mut self, _args: &[&str]) -> rusqlite::Result<()> {
        let sql = "SELECT seq , name , file FROM pragma_database_list";
        let conn = self.ctx.conn.borrow();
        let mut stmt = conn.prepare(sql)?;
        let col_count = stmt.column_count();

        let title = util::query_title_row(&mut stmt, col_count, self.ctx.mode)?;
        let data = util::query_data_rows(
            &mut stmt,
            col_count,
            self.ctx.mode,
            self.ctx.null_value_repr.as_ref(),
        )?;

        util::construct_and_print_output(
            &mut self.ctx.output,
            self.ctx.mode,
            title,
            data,
            self.ctx.with_header,
        );

        Ok(())
    }
    fn dot_dbconfig(&mut self, _args: &[&str]) {
        let attach_create = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_ATTACH_CREATE)
                .unwrap(),
        );
        let attach_write = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_ATTACH_WRITE)
                .unwrap(),
        );
        let comments = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_COMMENTS)
                .unwrap(),
        );
        let defensive = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_DEFENSIVE)
                .unwrap(),
        );
        let dps_ddl = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_DQS_DDL)
                .unwrap(),
        );
        let dps_dml = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_DQS_DML)
                .unwrap(),
        );
        let enable_fkey = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY)
                .unwrap(),
        );
        let enable_qpsg = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_QPSG)
                .unwrap(),
        );
        let enable_trigger = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_TRIGGER)
                .unwrap(),
        );
        let enable_view = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_VIEW)
                .unwrap(),
        );
        let fts3_tokenizer = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER)
                .unwrap(),
        );
        let legacy_alter_table = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_LEGACY_ALTER_TABLE)
                .unwrap(),
        );
        let legacy_file_format = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_LEGACY_FILE_FORMAT)
                .unwrap(),
        );
        let no_ckpt_on_close = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE)
                .unwrap(),
        );
        let reset_database = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_RESET_DATABASE)
                .unwrap(),
        );
        let reverse_scanorder = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_REVERSE_SCANORDER)
                .unwrap(),
        );
        let stmt_scanstatus = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_STMT_SCANSTATUS)
                .unwrap(),
        );
        let trigger_eqp = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_TRIGGER_EQP)
                .unwrap(),
        );
        let trusted_schema = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_TRUSTED_SCHEMA)
                .unwrap(),
        );
        let writable_schema = util::bool_to_on_or_off(
            self.ctx
                .conn
                .borrow()
                .db_config(DbConfig::SQLITE_DBCONFIG_WRITABLE_SCHEMA)
                .unwrap(),
        );
        let mut tbl = table! {
            ["attach_create", attach_create],
            ["attach_write", attach_write],
            ["comments", comments],
            ["defensive", defensive],
            ["dps_ddl", dps_ddl],
            ["dps_dml", dps_dml],
            ["enable_fkey", enable_fkey],
            ["enable_qpsg", enable_qpsg],
            ["enable_trigger", enable_trigger],
            ["enable_view", enable_view],
            ["fts3_tokenizer", fts3_tokenizer],
            ["legacy_alter_table", legacy_alter_table],
            ["legacy_file_format", legacy_file_format],
            // ["load_extension", load_extension],
            ["no_ckpt_on_close", no_ckpt_on_close],
            ["reset_database", reset_database],
            ["reverse_scanorder", reverse_scanorder],
            ["stmt_scanstatus", stmt_scanstatus],
            ["trigger_eqp", trigger_eqp],
            ["trusted_schema", trusted_schema],
            ["writable_schema", writable_schema]
        };
        tbl.set_titles(row![c => "pragmas", "value"]);

        let fmt = TableFormat::try_from(self.ctx.mode).unwrap_or(*crate::consts::BOX);
        tbl.set_format(fmt);

        self.ctx.output.print_prettytable(&mut tbl);
    }
    fn dot_dbinfo(&mut self, _args: &[&str]) {}
    fn dot_dbtotxt(&mut self, _args: &[&str]) {}
    fn dot_dump(&mut self, _args: &[&str]) {
        // cast this into a trait object to reduce duplicate code
        let writer: &mut dyn Write = match &mut self.ctx.output {
            Output::BufferedStdout(out) => out,
            Output::BufferedFile(f) => f,
        };

        // disable foreign keys constraint and create a transaction, this is important because
        // if the dumping process goes wrong, the results would be invalid. Therefore, we must put all of this operation
        // in a transaction block
        let _ = writeln!(writer, "PRAGMA foreign_keys=OFF;\nBEGIN TRANSACTION;");

        // get all things that the users creates
        let sql = "SELECT name, type, sql FROM sqlite_schema WHERE name NOT LIKE '%_autoindex_%'";
        let conn = self.ctx.conn.borrow();
        let mut items_stmt = conn
            .prepare(sql)
            .expect("unable to prepare query for dumping");
        let col_count = items_stmt.column_count();

        let created_items = util::query_data_rows(
            &mut items_stmt,
            col_count,
            TableMode::Box,
            self.ctx.null_value_repr.as_ref(),
        )
        .expect("unable to query sqlite_schema");

        // dump the sql representation of all things that the users creates
        for items in created_items {
            let item_name = &items[0];
            let item_type = &items[1];
            let item_sql = &items[2];
            // if the user creates a table or a view, we must populate the table with INSERT INTO statements if there are items inside it.
            if item_type == "table" || item_type == "view" {
                let table_to_populate_sql = format!("SELECT * FROM {item_name}");
                let conn = self.ctx.conn.borrow();
                let mut insert_into_stmt = conn
                    .prepare(&table_to_populate_sql)
                    .expect("unable to prepare query for populating data");

                let col_count = insert_into_stmt.column_count();

                let insert_datas = util::query_data_rows(
                    &mut insert_into_stmt,
                    col_count,
                    TableMode::Quote,
                    self.ctx.null_value_repr.as_ref(),
                )
                .expect("unable to get table content");

                for values in insert_datas {
                    let values = values.join(",");
                    let _ = writeln!(writer, "INSERT INTO {} VALUES ({});", item_name, values);
                }
            } else {
                // if its not a table, maybe an index or a trigger we could just print the sql.
                let _ = writeln!(writer, "{};", item_sql);
            }
        }

        // commit those changes
        let _ = writeln!(writer, "COMMIT;");
        // don't forget to flush!
        writer.flush().expect("unable to flush");
    }
    fn dot_echo(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".echo needs an argument");
            return;
        }

        let confirmation = util::on_or_off_to_bool(args[0]);
        self.ctx.with_echo = confirmation;
    }
    fn dot_eqp(&mut self, _args: &[&str]) {}
    fn dot_excel(&mut self, _args: &[&str]) {}
    fn dot_expert(&mut self, _args: &[&str]) {}
    fn dot_explain(&mut self, _args: &[&str]) {}
    fn dot_filectrl(&mut self, _args: &[&str]) {}
    fn dot_fullschema(&mut self, _args: &[&str]) {}
    fn dot_headers(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".headers needs at least an argument");
            return;
        }

        self.ctx.with_header = util::on_or_off_to_bool(args[0]);
    }
    fn dot_help(&mut self, _args: &[&str]) {
        let mut table = Table::new();
        // format titles with a center alignment based on `Prettytable::Cell::style_spec`
        table.set_titles(row![c => "command", "args", "desc"]);
        for [cmd, args, desc] in crate::consts::HELP_COMMANDS {
            table.add_row(row![cmd, args, desc]);
        }

        let fmt = TableFormat::try_from(self.ctx.mode).unwrap_or(*crate::consts::BOX);
        table.set_format(fmt);
        self.ctx.output.print_prettytable(&mut table);
    }
    fn dot_import(&mut self, _args: &[&str]) {}
    fn dot_imposter(&mut self, _args: &[&str]) {}
    fn dot_indexes(&mut self, _args: &[&str]) -> rusqlite::Result<()> {
        let sql =
            "SELECT name FROM sqlite_schema WHERE type = 'index' AND name NOT LIKE 'sqlite_%'";
        let conn = self.ctx.conn.borrow();
        let mut stmt = conn.prepare(sql)?;
        let col_count = stmt.column_count();

        let title = util::query_title_row(&mut stmt, col_count, self.ctx.mode)?;
        let table_names = util::query_data_rows(
            &mut stmt,
            col_count,
            self.ctx.mode,
            self.ctx.null_value_repr.as_ref(),
        )?;

        util::construct_and_print_output(
            &mut self.ctx.output,
            self.ctx.mode,
            title,
            table_names,
            true,
        );

        Ok(())
    }
    fn dot_intck(&mut self, _args: &[&str]) {}
    fn dot_limit(&mut self, _args: &[&str]) {}
    fn dot_lint(&mut self, _args: &[&str]) {}
    fn dot_load(&mut self, _args: &[&str]) {}
    fn dot_log(&mut self, _args: &[&str]) {}
    fn dot_mode(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".mode needs at least an argument");
            return;
        }

        self.ctx.mode = TableMode::try_from(args[0]).expect("unrecognized command");
    }
    fn dot_nonce(&mut self, _args: &[&str]) {}
    fn dot_nullvalue(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".nullvalue needs an argument");
            return;
        }

        self.ctx.null_value_repr = Some(args[0].to_string());
    }
    fn dot_once(&mut self, _args: &[&str]) {}
    fn dot_open(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".open needs an argument");
            return;
        }

        let path = Path::new(args[0]);
        self.ctx.cwd.push(path);

        let new_conn =
            Connection::open(&self.ctx.cwd).expect("unable to establish a new database connection");

        *self.ctx.conn.borrow_mut() = new_conn;

        self.ctx.cwd.pop();
    }
    fn dot_output(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".output needs at least on argument");
            return;
        }

        let path = Path::new(args[0]);
        self.ctx.cwd.push(path);

        if self.ctx.cwd.exists() {
            let f = File::open(&self.ctx.cwd).unwrap();
            let bufwriter = BufWriter::new(f);
            self.ctx.output = Output::BufferedFile(bufwriter);
            return;
        }

        let f = File::create(&self.ctx.cwd).expect("unable to create file");
        let bufwriter = BufWriter::new(f);
        self.ctx.output = Output::BufferedFile(bufwriter);

        self.ctx.cwd.pop();
    }
    fn dot_parameter(&mut self, _args: &[&str]) {}
    fn dot_print(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!();
            return;
        }

        println!("{}", args.join(" "));
    }
    fn dot_progress(&mut self, _args: &[&str]) {}
    fn dot_prompt(&mut self, _args: &[&str]) {}
    fn dot_read(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".read needs an argument");
            return;
        }

        let path = Path::new(args[0]);
        self.ctx.cwd.push(path);

        let db_file = File::open(&self.ctx.cwd).expect("unable to open file");
        let reader = BufReader::new(db_file);
        reader.split(b';').flatten().for_each(|sql| {
            let sql_str = str::from_utf8(&sql).expect("encountered a non-utf8 character");
            let trim = sql_str.trim();
            if !trim.is_empty() {
                self.run_user_query(trim).expect("unable to run user query");
            }
        });

        self.ctx.cwd.pop();
    }
    fn dot_recover(&mut self, _args: &[&str]) {}
    fn dot_restore(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".restore needs an argument");
            return;
        }

        let path = Path::new(args[0]);
        self.ctx.cwd.push(path);

        self.ctx
            .conn
            .borrow_mut()
            .restore(MAIN_DB, &self.ctx.cwd, Some(util::show_progress))
            .expect("unable to backup");

        self.ctx.cwd.pop();
    }
    fn dot_save(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".save or .backup needs an argument");
            return;
        }
        let path = Path::new(args[0]);
        self.ctx.cwd.push(path);

        self.ctx
            .conn
            .borrow()
            .backup(MAIN_DB, &self.ctx.cwd, Some(util::show_progress))
            .unwrap();

        self.ctx.cwd.pop();
    }
    fn dot_scanstats(&mut self, _args: &[&str]) {}
    fn dot_schema(&mut self, args: &[&str]) -> rusqlite::Result<()> {
        let mut sql = "SELECT sql FROM sqlite_schema WHERE name NOT LIKE '%_autoindex_%' ORDER BY tbl_name, type DESC, name".to_string();

        if !args.is_empty() {
            let item_name = args[0];
            sql = format!("SELECT sql FROM sqlite_schema WHERE name = '{}'", item_name);
        }

        let conn = self.ctx.conn.borrow();
        let mut stmt = conn.prepare(&sql)?;

        let table_names = util::query_data_rows(
            &mut stmt,
            1,
            TableMode::Box,
            self.ctx.null_value_repr.as_ref(),
        )?;

        let writer: &mut dyn Write = match &mut self.ctx.output {
            Output::BufferedStdout(out) => out,
            Output::BufferedFile(f) => f,
        };

        for table in table_names {
            writeln!(writer, "{};", table[0]).expect("unable to write all bytes");
        }

        writer
            .flush()
            .expect("unable to flush because not all bytes are written");

        Ok(())
    }
    fn dot_separator(&mut self, _args: &[&str]) {}
    fn dot_session(&mut self, _args: &[&str]) {}
    fn dot_sha3sum(&mut self, _args: &[&str]) {}
    fn dot_show(&mut self, _args: &[&str]) {}
    fn dot_stats(&mut self, _args: &[&str]) {}
    fn dot_system(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!(".system needs at least an argument");
            return;
        }

        let prog = args[0];
        let args = &args[1..];
        let output = Command::new(prog)
            .args(args)
            .output()
            .expect("unable to get output");

        let writer: &mut dyn Write = match &mut self.ctx.output {
            Output::BufferedStdout(out) => out,
            Output::BufferedFile(f) => f,
        };

        let _ = write!(
            writer,
            "{}",
            str::from_utf8(&output.stdout)
                .expect("unable to print output because theis one or more non-utf8 character")
        );
        let _ = write!(
            writer,
            "{}",
            str::from_utf8(&output.stderr)
                .expect("unable to print output because theis one or more non-utf8 character")
        );
    }
    fn dot_tables(&mut self, _args: &[&str]) -> rusqlite::Result<()> {
        let sql = "SELECT name FROM sqlite_schema WHERE type in ('table', 'view') AND name NOT LIKE 'sqlite_%' ORDER BY 1";
        let conn = self.ctx.conn.borrow();
        let mut stmt = conn.prepare(sql)?;
        let col_count = stmt.column_count();

        let title = util::query_title_row(&mut stmt, col_count, self.ctx.mode)?;
        let table_names = util::query_data_rows(
            &mut stmt,
            col_count,
            self.ctx.mode,
            self.ctx.null_value_repr.as_ref(),
        )?;

        util::construct_and_print_output(
            &mut self.ctx.output,
            self.ctx.mode,
            title,
            table_names,
            true,
        );

        Ok(())
    }
    fn dot_timeout(&mut self, _args: &[&str]) {}
    fn dot_timer(&mut self, _args: &[&str]) {}
    fn dot_trace(&mut self, _args: &[&str]) {}
    fn dot_unmodule(&mut self, _args: &[&str]) {}
    fn dot_version(&mut self, _args: &[&str]) {
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
        table.set_titles(row![c => "version", "date", "timestamp", "hash"]);
        let fmt = TableFormat::try_from(self.ctx.mode).unwrap_or(*crate::consts::BOX);
        table.set_format(fmt);

        self.ctx.output.print_prettytable(&mut table);
    }
    fn dot_vfsinfo(&mut self, _args: &[&str]) {}
    fn dot_vfslist(&mut self, _args: &[&str]) {}
    fn dot_vfsname(&mut self, _args: &[&str]) {}
    fn dot_width(&mut self, _args: &[&str]) {}
    fn dot_www(&mut self, _args: &[&str]) {}
}
