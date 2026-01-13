use crate::app::App;
use crate::tui::CustomEditor;
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

    let mut editor = CustomEditor::new();

    loop {
        let user_input = editor.readline();
        match user_input {
            Ok(input) => {
                let mut runner = CommandRunner::new(&mut ctx);
                runner.run_command(&input)?;
            }
            Err(e) => util::handle_readline_err(e),
        }
    }
}
