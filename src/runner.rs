use std::io::Write;

use crate::config::{Context, Output, TableMode};
use crate::util;
use prettytable::format::{Alignment, TableFormat};
use prettytable::{Cell, Row, Table};
use rusqlite::Statement;
use rusqlite::types::ValueRef;

pub struct CommandRunner<'a> {
    ctx: &'a mut Context,
}

impl<'a> CommandRunner<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx }
    }

    pub fn run_command(&mut self, input: &str) -> rusqlite::Result<()> {
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
            ".backup" => self.dot_backup(args),
            ".bail" => self.dot_bail(args),
            ".cd" => self.dot_cd(args),
            ".changes" => self.dot_changes(args),
            ".check" => self.dot_check(args),
            ".clone" => self.dot_clone(args),
            ".connection" => self.dot_connection(args),
            ".crlf" => self.dot_crlf(args),
            ".databases" => self.dot_databases(args),
            ".dbconfig" => self.dot_dbconfig(args),
            ".dbinfo" => self.dot_dbinfo(args),
            ".dbtotxt" => self.dot_dbtotxt(args),
            ".dump" => self.dot_dump(args),
            ".echo" => self.dot_echo(args),
            ".eqp" => self.dot_eqp(args),
            ".excel" => self.dot_excel(args),
            ".exit" => self.dot_exit(args),
            ".expert" => self.dot_expert(args),
            ".explain" => self.dot_explain(args),
            ".filectrl" => self.dot_filectrl(args),
            ".fullschema" => self.dot_fullschema(args),
            ".headers" => self.dot_headers(args),
            ".help" => self.dot_help(args),
            ".import" => self.dot_import(args),
            ".imposter" => self.dot_imposter(args),
            ".indexes" => self.dot_indexes(args),
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
            ".quit" => self.dot_quit(args),
            ".read" => self.dot_read(args),
            ".recover" => self.dot_recover(args),
            ".restore" => self.dot_restore(args),
            ".save" => self.dot_save(args),
            ".scanstats" => self.dot_scanstats(args),
            ".schema" => self.dot_schema(args),
            ".separator" => self.dot_separator(args),
            ".session" => self.dot_session(args),
            ".sha3sum" => self.dot_sha3sum(args),
            ".shell" => self.dot_shell(args),
            ".show" => self.dot_show(args),
            ".stats" => self.dot_stats(args),
            ".system" => self.dot_system(args),
            ".tables" => self.dot_tables(args),
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
        let mut stmt = self.ctx.conn.prepare(query)?;
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
            self.ctx.null_value.as_ref(),
        )?;

        util::construct_and_print_output(
            &mut self.ctx.output,
            self.ctx.mode,
            column_names,
            row_datas,
            self.ctx.with_header,
        );

        Ok(())
    }

    fn dot_archive(&mut self, args: &[&str]) {}
    fn dot_auth(&mut self, args: &[&str]) {}
    fn dot_backup(&mut self, args: &[&str]) {}
    fn dot_bail(&mut self, args: &[&str]) {}
    fn dot_cd(&mut self, args: &[&str]) {}
    fn dot_changes(&mut self, args: &[&str]) {}
    fn dot_check(&mut self, args: &[&str]) {}
    fn dot_clone(&mut self, args: &[&str]) {}
    fn dot_connection(&mut self, args: &[&str]) {}
    fn dot_crlf(&mut self, args: &[&str]) {}
    fn dot_databases(&mut self, args: &[&str]) {}
    fn dot_dbconfig(&mut self, args: &[&str]) {}
    fn dot_dbinfo(&mut self, args: &[&str]) {}
    fn dot_dbtotxt(&mut self, args: &[&str]) {}
    fn dot_dump(&mut self, args: &[&str]) {}
    fn dot_echo(&mut self, args: &[&str]) {}
    fn dot_eqp(&mut self, args: &[&str]) {}
    fn dot_excel(&mut self, args: &[&str]) {}
    fn dot_exit(&mut self, args: &[&str]) {}
    fn dot_expert(&mut self, args: &[&str]) {}
    fn dot_explain(&mut self, args: &[&str]) {}
    fn dot_filectrl(&mut self, args: &[&str]) {}
    fn dot_fullschema(&mut self, args: &[&str]) {}
    fn dot_headers(&mut self, args: &[&str]) {}
    fn dot_help(&mut self, args: &[&str]) {}
    fn dot_import(&mut self, args: &[&str]) {}
    fn dot_imposter(&mut self, args: &[&str]) {}
    fn dot_indexes(&mut self, args: &[&str]) {}
    fn dot_intck(&mut self, args: &[&str]) {}
    fn dot_limit(&mut self, args: &[&str]) {}
    fn dot_lint(&mut self, args: &[&str]) {}
    fn dot_load(&mut self, args: &[&str]) {}
    fn dot_log(&mut self, args: &[&str]) {}
    fn dot_mode(&mut self, args: &[&str]) {}
    fn dot_nonce(&mut self, args: &[&str]) {}
    fn dot_nullvalue(&mut self, args: &[&str]) {}
    fn dot_once(&mut self, args: &[&str]) {}
    fn dot_open(&mut self, args: &[&str]) {}
    fn dot_output(&mut self, args: &[&str]) {}
    fn dot_parameter(&mut self, args: &[&str]) {}
    fn dot_print(&mut self, args: &[&str]) {}
    fn dot_progress(&mut self, args: &[&str]) {}
    fn dot_prompt(&mut self, args: &[&str]) {}
    fn dot_quit(&mut self, args: &[&str]) {}
    fn dot_read(&mut self, args: &[&str]) {}
    fn dot_recover(&mut self, args: &[&str]) {}
    fn dot_restore(&mut self, args: &[&str]) {}
    fn dot_save(&mut self, args: &[&str]) {}
    fn dot_scanstats(&mut self, args: &[&str]) {}
    fn dot_schema(&mut self, args: &[&str]) {}
    fn dot_separator(&mut self, args: &[&str]) {}
    fn dot_session(&mut self, args: &[&str]) {}
    fn dot_sha3sum(&mut self, args: &[&str]) {}
    fn dot_shell(&mut self, args: &[&str]) {}
    fn dot_show(&mut self, args: &[&str]) {}
    fn dot_stats(&mut self, args: &[&str]) {}
    fn dot_system(&mut self, args: &[&str]) {}
    fn dot_tables(&mut self, args: &[&str]) {}
    fn dot_timeout(&mut self, args: &[&str]) {}
    fn dot_timer(&mut self, args: &[&str]) {}
    fn dot_trace(&mut self, args: &[&str]) {}
    fn dot_unmodule(&mut self, args: &[&str]) {}
    fn dot_version(&mut self, args: &[&str]) {}
    fn dot_vfsinfo(&mut self, args: &[&str]) {}
    fn dot_vfslist(&mut self, args: &[&str]) {}
    fn dot_vfsname(&mut self, args: &[&str]) {}
    fn dot_width(&mut self, args: &[&str]) {}
    fn dot_www(&mut self, args: &[&str]) {}
}
