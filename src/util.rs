use prettytable::format::Alignment;
use prettytable::{Cell, Row};
use rusqlite::Statement;
use std::io::Write;

use rusqlite::types::ValueRef;

use crate::shqlite::Output;

pub fn value_ref_to_str(valref: ValueRef) -> String {
    match valref {
        ValueRef::Null => "null".to_string(),
        ValueRef::Integer(i) => i.to_string(),
        ValueRef::Real(r) => r.to_string(),
        ValueRef::Blob(blob) => {
            format!("<BLOB {} bytes>", blob.len())
        }
        ValueRef::Text(txt) => {
            let str_utf = str::from_utf8(txt).expect("UTF-8 error blob string conversion");
            format!("'{}'", str_utf.to_string())
        }
    }
}

pub fn write_generic_sql_stmt(writer: &mut Output, sql_stmt: String) {
    match writer {
        Output::BufferedStdout(out) => {
            let _ = writeln!(out, "{};", sql_stmt);
        }
        Output::BufferedFile(file) => {
            let _ = writeln!(file, "{};", sql_stmt);
        }
    }
}

pub fn write_insert_stmt(writer: &mut Output, table_name: &str, values: Vec<String>) {
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

pub fn query_colname_as_rows(stmt: &Statement, col_count: usize) -> Row {
    let mut names: Vec<Cell> = Vec::with_capacity(col_count);
    for idx in 0..col_count {
        let name = stmt.column_name(idx).expect("index of out bounds");
        let cell = Cell::new_align(name, Alignment::CENTER);
        names.push(cell);
    }
    Row::new(names)
}

pub fn query_data_as_rows(stmt: &mut Statement) -> Vec<Row> {
    let col_count = stmt.column_count();
    let rows = stmt
        .query_map([], |row| {
            let mut values: Vec<Cell> = Vec::with_capacity(col_count);
            for idx in 0..col_count {
                let valref = row.get_ref(idx).expect("index out of bounds");
                let stringify = value_ref_to_str(valref);
                let cell = Cell::new(&stringify);
                values.push(cell);
            }
            let to_row = Row::new(values);
            Ok(to_row)
        })
        .expect("unable to `query_map` all fields");

    rows.flatten().collect()
}
