use prettytable::format::Alignment;
use prettytable::{Cell, Row};
use rusqlite::Statement;
use std::io::Write;

use rusqlite::types::ValueRef;

use crate::shqlite::Output;

pub fn onoff_to_bool(arg: &str) -> bool {
    match arg {
        "on" => true,
        "off" => false,
        _ => {
            println!("ERROR: Not a boolean value: \"{}\".", arg);
            false
        }
    }
}

pub fn bool_to_onoff(b: bool) -> &'static str {
    match b {
        true => "on",
        false => "off",
    }
}

pub fn value_ref_to_str(valref: ValueRef, with_quotes: bool, null_value: Option<&str>) -> String {
    match valref {
        ValueRef::Null => null_value.unwrap_or("").to_string(),
        ValueRef::Integer(i) => i.to_string(),
        ValueRef::Real(r) => r.to_string(),
        ValueRef::Blob(blob) => {
            format!("<BLOB {} bytes>", blob.len())
        }
        ValueRef::Text(txt) => {
            let str_utf = str::from_utf8(txt).expect("UTF-8 error blob string conversion");
            if with_quotes {
                return format!("'{}'", str_utf);
            }
            format!("{}", str_utf)
        }
    }
}

pub fn write_generic_sql_stmt(writer: &mut Output, sql_stmt: &str) {
    match writer {
        Output::BufferedStdout(out) => {
            let _ = writeln!(out, "{};", sql_stmt);
        }
        Output::BufferedFile(file) => {
            let _ = writeln!(file, "{};", sql_stmt);
        }
    }
}

pub fn write_insert_stmt(writer: &mut Output, table_name: &str, values: &Vec<String>) {
    let joined = values.join(", ");
    match writer {
        Output::BufferedStdout(out) => {
            let _ = writeln!(out, "INSERT INTO {} VALUES ({});", table_name, joined);
        }
        Output::BufferedFile(file) => {
            let _ = writeln!(file, "INSERT INTO {} VALUES ({});", table_name, joined);
        }
    }
}

pub fn query_colname_as_tbl_rows(stmt: &Statement, col_count: usize) -> Row {
    let mut names: Vec<Cell> = Vec::with_capacity(col_count);
    for idx in 0..col_count {
        let name = stmt.column_name(idx).expect("index of out bounds");
        let cell = Cell::new_align(name, Alignment::CENTER);
        names.push(cell);
    }
    Row::new(names)
}

pub fn query_data_as_tbl_rows(
    stmt: &mut Statement,
    col_count: usize,
    with_quotes: bool,
    null_value: Option<&str>,
) -> Vec<Row> {
    let rows = stmt
        .query_map([], |row| {
            let mut values: Vec<Cell> = Vec::with_capacity(col_count);
            for idx in 0..col_count {
                let valref = row.get_ref(idx).expect("index out of bounds");
                let stringify = value_ref_to_str(valref, with_quotes, null_value);
                let cell = Cell::new(&stringify);
                values.push(cell);
            }
            let to_row = Row::new(values);
            Ok(to_row)
        })
        .expect("unable to `query_map` all fields");

    rows.flatten().collect()
}

pub fn query_data_as_str(
    stmt: &mut Statement,
    col_count: usize,
    with_quotes: bool,
    null_value: Option<&str>,
) -> Vec<Vec<String>> {
    let rows = stmt
        .query_map([], |row| {
            let mut values: Vec<String> = Vec::with_capacity(col_count);
            for idx in 0..col_count {
                let valref = row.get_ref(idx).expect("index out of bounds");
                let stringify = value_ref_to_str(valref, with_quotes, null_value);
                values.push(stringify);
            }
            Ok(values)
        })
        .expect("unable to `query_map` all fields");

    rows.flatten().collect()
}
