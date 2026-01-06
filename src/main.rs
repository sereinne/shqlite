pub mod app;
pub mod shqlite;

use crate::app::App;
use crate::shqlite::Shqlite;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let app = App::parse();
    let mut shqlite = Shqlite::from(app);
    shqlite.start_repl()?;
    Ok(())
}
