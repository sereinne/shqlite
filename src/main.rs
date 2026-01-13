use crate::app::App;
use crate::{config::Context, runner::CommandRunner};
use clap::Parser;
use rustyline::DefaultEditor;

mod app;
mod config;
mod consts;
mod runner;
mod shqlite;
mod util;

fn main() -> anyhow::Result<()> {
    let app = App::parse();

    let mut ctx = Context::from(app);

    let mut editor = DefaultEditor::new()?;

    loop {
        let user_input = editor.readline("shqlite> ");
        match user_input {
            Ok(input) => {
                let mut runner = CommandRunner::new(&mut ctx);
                runner.run_command(&input)?;
            }
            Err(e) => util::handle_readline_err(e),
        }
    }
}
