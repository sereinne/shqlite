use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};
use prettytable::{Cell, Row, Table};
use rusqlite::Connection;
use rusqlite::types::ValueRef;
use rustyline::DefaultEditor;
use std::path::Path;

// MSRV without prettytable: 1.88
pub struct Shqlite<'item> {
    db_path: Option<&'item Path>,
    db_conn: Connection,
}

impl<'item> Shqlite<'item> {
    pub fn new() -> Self {
        return Self {
            db_path: None,
            db_conn: Connection::open_in_memory()
                .expect("could not establish a temporary database connection"),
        };
    }

    pub fn with_db(file_path: &'item str) -> Self {
        let path = Path::new(file_path);
        return Self {
            db_path: Some(path),
            db_conn: Connection::open(path)
                .expect("could not establish a persistent database connection"),
        };
    }

    pub fn start_repl(self: &mut Self) -> anyhow::Result<()> {
        let mut editor = DefaultEditor::new()?;
        loop {
            let user_input = editor.readline("shqlite> ")?;
            if user_input.starts_with(".quit") {
                break;
            } else if user_input.starts_with(".") {
                self.execute_dot_command(&user_input);
            } else {
                self.execute_user_query(&user_input);
            }
        }
        Ok(())
    }

    fn execute_user_query(self: &mut Self, query: &str) {
        if query.starts_with("SELECT") {
            let mut stmt = self
                .db_conn
                .prepare(query)
                .expect("unable to create a prepared statement");
            let col_count = stmt.column_count();

            let mut columns: Vec<Cell> = Vec::with_capacity(col_count);

            for col_idx in 0..col_count {
                let colname = stmt
                    .column_name(col_idx)
                    .expect("out of bound index of column");
                columns.push(Cell::new(colname));
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
            let format = FormatBuilder::new()
                .column_separator('│')
                .separator(LinePosition::Intern, LineSeparator::new('─', '┼', '├', '┤'))
                .padding(1, 1)
                .separator(LinePosition::Title, LineSeparator::new('─', '┴', '┤', '├'))
                .separator(LinePosition::Bottom, LineSeparator::new('─', '┴', '╰', '╯'))
                .separator(LinePosition::Top, LineSeparator::new('─', '┬', '╭', '╮'))
                .borders('│')
                .build();

            table.set_format(format);
            // first row always contains the column name
            table.add_row(Row::new(columns));
            // the rest contains the value of that column
            for row_result in mapped_rows {
                if let Ok(row) = row_result {
                    table.add_row(Row::new(row));
                }
            }

            table.printstd();
        } else {
        }
    }

    fn execute_dot_command(self: &mut Self, user_input: &str) {
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
            ".databases" => Self::dot_databases(dot_cmd_args),
            ".dbconfig" => Self::dot_dbconfig(dot_cmd_args),
            ".dbinfo" => Self::dot_dbinfo(dot_cmd_args),
            ".dbtotxt" => Self::dot_dbtotxt(dot_cmd_args),
            ".dump" => Self::dot_dump(dot_cmd_args),
            ".echo" => Self::dot_echo(dot_cmd_args),
            ".eqp" => Self::dot_eqp(dot_cmd_args),
            ".excel" => Self::dot_excel(dot_cmd_args),
            ".exit" => Self::dot_exit(dot_cmd_args),
            ".expert" => Self::dot_expert(dot_cmd_args),
            ".explain" => Self::dot_explain(dot_cmd_args),
            ".filectrl" => Self::dot_filectrl(dot_cmd_args),
            ".fullschema" => Self::dot_fullschema(dot_cmd_args),
            ".headers" => Self::dot_headers(dot_cmd_args),
            ".help" => Self::dot_help(dot_cmd_args),
            ".import" => Self::dot_import(dot_cmd_args),
            ".imposter" => Self::dot_imposter(dot_cmd_args),
            ".indexes" => Self::dot_indexes(dot_cmd_args),
            ".intck" => Self::dot_intck(dot_cmd_args),
            ".limit" => Self::dot_limit(dot_cmd_args),
            ".lint" => Self::dot_lint(dot_cmd_args),
            ".load" => Self::dot_load(dot_cmd_args),
            ".log" => Self::dot_log(dot_cmd_args),
            ".mode" => Self::dot_mode(dot_cmd_args),
            ".nonce" => Self::dot_nonce(dot_cmd_args),
            ".nullvalue" => Self::dot_nullvalue(dot_cmd_args),
            ".once" => Self::dot_once(dot_cmd_args),
            ".open" => Self::dot_open(dot_cmd_args),
            ".output" => Self::dot_output(dot_cmd_args),
            ".parameter" => Self::dot_parameter(dot_cmd_args),
            ".print" => Self::dot_print(dot_cmd_args),
            ".progress" => Self::dot_progress(dot_cmd_args),
            ".prompt" => Self::dot_prompt(dot_cmd_args),
            ".quit" => Self::dot_quit(dot_cmd_args),
            ".read" => Self::dot_read(dot_cmd_args),
            ".recover" => Self::dot_recover(dot_cmd_args),
            ".restore" => Self::dot_restore(dot_cmd_args),
            ".save" => Self::dot_save(dot_cmd_args),
            ".scanstats" => Self::dot_scanstats(dot_cmd_args),
            ".schema" => Self::dot_schema(dot_cmd_args),
            ".separator" => Self::dot_separator(dot_cmd_args),
            ".session" => Self::dot_session(dot_cmd_args),
            ".sha3sum" => Self::dot_sha3sum(dot_cmd_args),
            ".shell" => Self::dot_shell(dot_cmd_args),
            ".show" => Self::dot_show(dot_cmd_args),
            ".stats" => Self::dot_stats(dot_cmd_args),
            ".system" => Self::dot_system(dot_cmd_args),
            ".tables" => Self::dot_tables(dot_cmd_args),
            ".timeout" => Self::dot_timeout(dot_cmd_args),
            ".timer" => Self::dot_timer(dot_cmd_args),
            ".trace" => Self::dot_trace(dot_cmd_args),
            ".unmodule" => Self::dot_unmodule(dot_cmd_args),
            ".version" => Self::dot_version(dot_cmd_args),
            ".vfsinfo" => Self::dot_vfsinfo(dot_cmd_args),
            ".vfslist" => Self::dot_vfslist(dot_cmd_args),
            ".vfsname" => Self::dot_vfsname(dot_cmd_args),
            ".width" => Self::dot_width(dot_cmd_args),
            ".www" => Self::dot_www(dot_cmd_args),
            _ => {}
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
    fn dot_cd(_args: &[&str]) {
        todo!("WIP to implement dot_cd function")
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
    fn dot_databases(_args: &[&str]) {
        todo!("WIP to implement dot_databases function")
    }
    fn dot_dbconfig(_args: &[&str]) {
        todo!("WIP to implement dot_dbconfig function")
    }
    fn dot_dbinfo(_args: &[&str]) {
        todo!("WIP to implement dot_dbinfo function")
    }
    fn dot_dbtotxt(_args: &[&str]) {
        todo!("WIP to implement dot_dbtotxt function")
    }
    fn dot_dump(_args: &[&str]) {
        todo!("WIP to implement dot_dump function")
    }
    fn dot_echo(_args: &[&str]) {
        todo!("WIP to implement dot_echo function")
    }
    fn dot_eqp(_args: &[&str]) {
        todo!("WIP to implement dot_eqp function")
    }
    fn dot_excel(_args: &[&str]) {
        todo!("WIP to implement dot_excel function")
    }
    fn dot_exit(_args: &[&str]) {
        todo!("WIP to implement dot_exit function")
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
    fn dot_help(_args: &[&str]) {
        todo!("WIP to implement dot_help function")
    }
    fn dot_import(_args: &[&str]) {
        todo!("WIP to implement dot_import function")
    }
    fn dot_imposter(_args: &[&str]) {
        todo!("WIP to implement dot_imposter function")
    }
    fn dot_indexes(_args: &[&str]) {
        todo!("WIP to implement dot_indexes function")
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
    fn dot_mode(_args: &[&str]) {
        todo!("WIP to implement dot_mode function")
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
    fn dot_open(_args: &[&str]) {
        todo!("WIP to implement dot_open function")
    }
    fn dot_output(_args: &[&str]) {
        todo!("WIP to implement dot_output function")
    }
    fn dot_parameter(_args: &[&str]) {
        todo!("WIP to implement dot_parameter function")
    }
    fn dot_print(_args: &[&str]) {
        todo!("WIP to implement dot_print function")
    }
    fn dot_progress(_args: &[&str]) {
        todo!("WIP to implement dot_progress function")
    }
    fn dot_prompt(_args: &[&str]) {
        todo!("WIP to implement dot_prompt function")
    }
    fn dot_quit(_args: &[&str]) {
        todo!("WIP to implement dot_quit function")
    }
    fn dot_read(_args: &[&str]) {
        todo!("WIP to implement dot_read function")
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
    fn dot_schema(_args: &[&str]) {
        todo!("WIP to implement dot_schema function")
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
    fn dot_shell(_args: &[&str]) {
        todo!("WIP to implement dot_shell function")
    }
    fn dot_show(_args: &[&str]) {
        todo!("WIP to implement dot_show function")
    }
    fn dot_stats(_args: &[&str]) {
        todo!("WIP to implement dot_stats function")
    }
    fn dot_system(_args: &[&str]) {
        todo!("WIP to implement dot_system function")
    }
    fn dot_tables(_args: &[&str]) {
        todo!("WIP to implement dot_tables function")
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
    fn dot_version(_args: &[&str]) {
        todo!("WIP to implement dot_version function")
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
    let mut shqlite = Shqlite::with_db("./mahasiswa.db");
    shqlite.start_repl()?;
    Ok(())
}
