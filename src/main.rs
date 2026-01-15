use std::process::exit;

use crate::app::App;
use crate::tui::Prompt;
use crate::{config::Context, runner::CommandRunner};
use clap::Parser;

mod app;
mod config;
mod consts;
mod runner;
mod tui;
mod util;

fn main() -> anyhow::Result<()> {
    let app = App::parse();

    let mut ctx = Context::from(app);
    let cloned_conn = ctx.conn.clone();

    let mut prompt = Prompt::new(cloned_conn);

    loop {
        let user_input = prompt.readline();
        match user_input {
            Ok(input) => {
                prompt.add_history_entry(&input)?;
                if input == ".quit" {
                    break;
                }

                if input.starts_with(".exit") {
                    let exit_code = input
                        .split(" ")
                        .skip(1)
                        .take(1)
                        .next()
                        .unwrap_or("0")
                        .parse::<i32>()
                        .unwrap_or(0);
                    exit(exit_code);
                }

                let mut runner = CommandRunner::new(&mut ctx);
                runner.run_command(&input)?;
            }
            Err(e) => util::handle_readline_err(e),
        }
    }
    prompt.save_history()?;
    Ok(())
}
