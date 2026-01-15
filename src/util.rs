use crate::config::TableMode;
use prettytable::format::Alignment;
use prettytable::format::TableFormat;
use prettytable::{Cell, Row, Table};
use rusqlite::Statement;
use rusqlite::backup::Progress;
use rustyline::error::ReadlineError;
use std::io::Write;
use std::process::exit;

use rusqlite::types::ValueRef;

use crate::config::Output;

pub fn tokenize(input: &str) -> Vec<&str> {
    input.split(|c: char| c.is_whitespace()).collect()
}

pub fn should_complete_tables(tokens: &[&str]) -> bool {
    if tokens.len() < 2 {
        return false;
    }

    let prev_word = tokens[tokens.len() - 2];
    matches!(prev_word, "JOIN" | "FROM")
}

pub fn should_complete_columns(tokens: &[&str]) -> bool {
    if tokens.len() < 2 {
        return false;
    }

    let prev_word = tokens[tokens.len() - 2];
    matches!(prev_word, "SELECT" | "WHERE" | "ORDER BY" | "ON")
}

pub fn show_progress(prog: Progress) {
    let completed = prog.pagecount - prog.remaining;
    let percent = (completed as f64 / prog.pagecount as f64 * 100.0) as u32;
    let bar_width = 50;
    let filled = (bar_width * completed) / prog.pagecount;

    let bar: String = (0..bar_width)
        .map(|i| if i < filled as i32 { '█' } else { '░' })
        .collect();

    print!(
        "\r[{}] {}% ({}/{})",
        bar, percent, completed, prog.pagecount
    );
    std::io::stdout().flush().unwrap();
}

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

pub fn on_or_off_to_bool(on_or_off: &str) -> bool {
    match on_or_off {
        "on" => true,
        "off" => false,
        _ => {
            println!("unexpected input, input must be either on or off. Returning false");
            false
        }
    }
}

pub fn bool_to_on_or_off(b: bool) -> &'static str {
    match b {
        true => "on",
        false => "off",
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
        TableMode::Ascii => print_ugly_ascii(writer, data, title, with_header),
        TableMode::Html => print_fmtted_html(writer, data, title),
        TableMode::Insert => print_insert(writer, data, title),
        TableMode::Json => print_fmtted_json(writer, data, title),
        TableMode::Line => print_line_mode(writer, data, title),
        // else `prettytable` is able to print. even though `prettytable` could print html like
        _ => {
            // populate the table with the row data
            let mut table = Table::from(data);

            // get the title row and append it to the first row in the table
            // this must be collected into a `Vec<Cell>` instead of `Row` because
            // the styling will not take effect
            let centered_title: Vec<Cell> = title
                .into_iter()
                .map(|colname| Cell::new_align(&colname, Alignment::CENTER))
                .collect();

            // appends the title based on this rule
            match mode {
                TableMode::List | TableMode::Csv | TableMode::Tabs => {
                    if with_header {
                        table.set_titles(Row::new(centered_title));
                    } else {
                        table.unset_titles();
                    }
                }
                _ => table.set_titles(Row::new(centered_title)),
            }

            // formats the table based on `mode`
            let fmt = TableFormat::try_from(mode).unwrap_or(*crate::consts::BOX);
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
            TableMode::Quote | TableMode::Insert => format!("'{}'", str::from_utf8(txt).unwrap()),
            TableMode::Tcl => format!("\"{}\"", str::from_utf8(txt).unwrap()),
            _ => str::from_utf8(txt)
                .expect("encountered an invalid character")
                .to_string(),
        },
        ValueRef::Blob(blob) => format!("<BLOB {} bytes>", blob.len()),
    }
}

fn print_fmtted_json(writer: &mut Output, data: Vec<Vec<String>>, title: Vec<String>) {
    // cast this into a trait object to reduce duplicate code
    let writer: &mut dyn Write = match writer {
        Output::BufferedStdout(out) => out,
        Output::BufferedFile(f) => f,
    };

    // the length of all data rows
    let total_data_rows = data.len();
    // total cells of each rows (title and data)
    let total_cells = title.len();

    // always start with an array of json
    let _ = writeln!(writer, "[");
    // print each data row
    for data_idx in 0..total_data_rows {
        // three spaces for json object
        let _ = writeln!(writer, "   {{");
        // for each data row, print each cells with its title
        // it is guaranteed that the length of each data row is equal to the length of title
        for cell_idx in 0..total_cells {
            // five spaces for json fields
            let _ = writeln!(
                writer,
                "     \"{}\": {},",
                title[cell_idx], data[data_idx][cell_idx]
            );
            // if we are at the last field, we omit the last comma for a valid json object
            if cell_idx == total_cells - 1 {
                let _ = writeln!(
                    writer,
                    "     \"{}\": {}",
                    title[cell_idx], data[data_idx][cell_idx]
                );
                continue;
            }
        }
        // if we are at the last object, we omit the last comma for a valid json object
        if data_idx == total_data_rows - 1 {
            let _ = writeln!(writer, "   }}");
            continue;
        }
        let _ = writeln!(writer, "   }},");
    }
    let _ = writeln!(writer, "]");
    writer.flush().expect("unable to flush ");
}

fn print_fmtted_html(writer: &mut Output, data: Vec<Vec<String>>, title: Vec<String>) {
    // cast this into a trait object to reduce duplicate code
    let writer: &mut dyn Write = match writer {
        Output::BufferedStdout(out) => out,
        Output::BufferedFile(f) => f,
    };

    // the length of all data rows
    let total_data_rows = data.len();
    // total cells of each rows (title and data)
    let total_cells = title.len();

    // print the headers first which contains the title name for each data
    let _ = writeln!(writer, "<tr>");

    for title_cell_idx in 0..total_cells {
        let _ = writeln!(writer, "    <th>{}</th>", title[title_cell_idx]);
    }

    let _ = writeln!(writer, "</tr>");

    // print the content that matches the "schema" that the header

    for data_idx in 0..total_data_rows {
        let _ = writeln!(writer, "<tr>");
        for cell_idx in 0..total_cells {
            let _ = writeln!(writer, "    <td>{}</td>", data[data_idx][cell_idx]);
        }
        let _ = writeln!(writer, "</tr>");
    }

    writer.flush().expect("unable to flush");
}

fn print_insert(writer: &mut Output, datas: Vec<Vec<String>>, title: Vec<String>) {
    // cast this into a trait object to reduce duplicate code
    let writer: &mut dyn Write = match writer {
        Output::BufferedStdout(out) => out,
        Output::BufferedFile(f) => f,
    };

    for data in datas {
        let data_joined = data.join(",");
        let title_joined = title.join(",");
        let _ = writeln!(
            writer,
            "INSERT INTO \"table\" ({}) VALUES ({});",
            title_joined, data_joined
        );
    }

    writer.flush().expect("unable to flush");
}

fn print_ugly_ascii(
    writer: &mut Output,
    datas: Vec<Vec<String>>,
    title: Vec<String>,
    with_header: bool,
) {
    // cast this into a trait object to reduce duplicate code
    let writer: &mut dyn Write = match writer {
        Output::BufferedStdout(out) => out,
        Output::BufferedFile(f) => f,
    };

    // default capacity
    let mut final_string = String::with_capacity(2048);

    if with_header {
        let joined_title = title.join("");
        final_string.push_str(&joined_title);
    }

    for data in datas {
        let joined = data.join("");
        final_string.push_str(&joined);
    }

    let _ = writeln!(writer, "{}", final_string);
    writer.flush().expect("unable flush");
}

fn print_line_mode(writer: &mut Output, datas: Vec<Vec<String>>, title: Vec<String>) {
    // cast this into a trait object to reduce duplicate code
    let writer: &mut dyn Write = match writer {
        Output::BufferedStdout(out) => out,
        Output::BufferedFile(f) => f,
    };

    let max_len = title.iter().map(|entry| entry.len()).max().unwrap_or(0);

    for data in datas {
        for (i, value) in data.iter().enumerate() {
            if i < value.len() {
                let _ = writeln!(writer, "{:>width$} = {}", title[i], value, width = max_len);
            }
        }
    }
}
