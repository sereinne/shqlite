use std::path::PathBuf;

use rustyline::history::FileHistory;
use rustyline::{Config, EditMode, Editor};

pub struct CustomEditor {
    history_file: PathBuf,
    editor: Editor<(), FileHistory>,
}

impl CustomEditor {
    pub fn new() -> Self {
        let history_config = Config::builder()
            .history_ignore_dups(true)
            .unwrap()
            .history_ignore_space(true)
            .max_history_size(2048)
            .unwrap()
            .build();
        let editor_config = Config::builder()
            .edit_mode(EditMode::Vi)
            .auto_add_history(true)
            .build();
        let history = FileHistory::with_config(&history_config);
        let mut history_file_path = std::env::home_dir().expect("unable to get users home dir");
        history_file_path.push(".shqlite_hist");

        let editor =
            Editor::with_history(editor_config, history).expect("unable to create a line editor");

        Self {
            editor: editor,
            history_file: history_file_path,
        }
    }

    pub fn readline(&mut self) -> rustyline::Result<String> {
        let res = self.editor.readline("shqlite> ");
        self.append_history();
        res
    }

    pub fn append_history(&mut self) {
        self.editor
            .append_history(&self.history_file)
            .expect("unable to append to history file");
    }
}
