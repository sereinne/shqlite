use crate::config::TableMode;
use prettytable::format::Alignment;
use prettytable::format::TableFormat;
use prettytable::{Cell, Row, Table};
use rusqlite::Statement;
use rustyline::error::ReadlineError;
use std::io::Write;
use std::process::exit;

use rusqlite::types::ValueRef;

use crate::config::Output;

pub fn handle_readline_err(err: ReadlineError) {
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

pub fn construct_and_print_output(
    writer: &mut Output,
    mode: TableMode,
    title: Vec<String>,
    data: Vec<Vec<String>>,
    with_header: bool,
) {
    match mode {
        // these modes can't be constructed using the `prettytable` crate
        TableMode::Ascii => {}
        TableMode::Html => {}
        TableMode::Insert => {}
        TableMode::Json => {}
        TableMode::Line => {}
        // else `prettytable` is able to print. even though `prettytable` could print html like
        _ => {
            // populate the table with the row data
            let mut table = Table::from(data);

            // get the title row and append it to the first row in the table
            let centered_title = title
                .into_iter()
                .map(|colname| Cell::new_align(&colname, Alignment::CENTER))
                .collect();

            // appends the title based on this rule
            match mode {
                TableMode::List | TableMode::Csv | TableMode::Tabs => {
                    if with_header {
                        table.set_titles(centered_title);
                    } else {
                        table.unset_titles();
                    }
                }
                _ => table.set_titles(centered_title),
            }

            // formats the table based on `mode`
            let fmt = TableFormat::from(mode);
            table.set_format(fmt);

            // prints the table
            writer.print_prettytable(&mut table);
            writer.flush();
        }
    }
}

pub fn query_title_row(
    stmt: &mut Statement,
    col_count: usize,
    mode: TableMode,
) -> rusqlite::Result<Vec<String>> {
    let mut titles: Vec<String> = Vec::with_capacity(col_count);

    for col_idx in 0..col_count {
        let title = match mode {
            TableMode::Quote => format!("'{}'", stmt.column_name(col_idx)?),
            TableMode::Tcl => format!("\"{}\"", stmt.column_name(col_idx)?),
            _ => format!("{}", stmt.column_name(col_idx)?),
        };
        titles.push(title);
    }

    Ok(titles)
}

pub fn query_data_rows(
    stmt: &mut Statement,
    col_count: usize,
    mode: TableMode,
    null_value: Option<&String>,
) -> rusqlite::Result<Vec<Vec<String>>> {
    let rows = stmt.query_map((), |row| {
        let mut data = Vec::with_capacity(col_count);
        for col_idx in 0..col_count {
            let valref = row.get_ref(col_idx)?;
            let stringified = parse_sql_value(valref, mode, null_value);
            data.push(stringified);
        }
        Ok(data)
    })?;
    rows.collect()
}

fn parse_sql_value(sql_val: ValueRef, mode: TableMode, null_value: Option<&String>) -> String {
    let null_value = null_value.cloned().unwrap_or(String::new());
    match sql_val {
        ValueRef::Null => null_value,
        ValueRef::Integer(i) => i.to_string(),
        ValueRef::Real(fp) => fp.to_string(),
        ValueRef::Text(txt) => match mode {
            TableMode::Quote => format!("'{}'", str::from_utf8(txt).unwrap()),
            TableMode::Tcl => format!("\"{}\"", str::from_utf8(txt).unwrap()),
            _ => str::from_utf8(txt)
                .expect("encountered an invalid character")
                .to_string(),
        },
        ValueRef::Blob(blob) => format!("<BLOB {} bytes>", blob.len()),
    }
}
