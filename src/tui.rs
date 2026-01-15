use crate::consts;
use radix_trie::{Trie, TrieCommon};
use rustyline::Helper;
use rustyline::completion::Completer;
use rustyline::config::Configurer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Config, EditMode, Editor, history::FileHistory};
use std::borrow::{Borrow, Cow};
use std::fs::read_to_string;
use std::ops::Add;
use std::path::PathBuf;

use crate::consts::HELP_COMMANDS;

pub struct PromptCompleter<'a> {
    // conn: Rc<RefCell<Connection>>,
    dot_commands: Trie<&'a str, ()>,
}

impl<'a> PromptCompleter<'a> {
    pub fn new() -> Self {
        let mut rdx = Trie::new();

        HELP_COMMANDS.iter().for_each(|info| {
            rdx.insert(info[0], ());
        });

        Self { dot_commands: rdx }
    }
}

impl<'a> Helper for PromptCompleter<'a> {}

impl<'a> Completer for PromptCompleter<'a> {
    type Candidate = &'a str;
    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        if let Some(children) = self.dot_commands.get_raw_descendant(line) {
            let candidates = children.keys().map(|key| *key).collect::<Vec<&str>>();
            return Ok((0, candidates));
        }
        Ok((0, vec![]))
    }
}

const GRUVBOX_RED: &str = "\x1b[38;5;167m"; // Keywords
const GRUVBOX_GREEN: &str = "\x1b[38;5;142m"; // Strings
const GRUVBOX_YELLOW: &str = "\x1b[38;5;214m"; // Functions
const GRUVBOX_BLUE: &str = "\x1b[38;5;109m"; // Numbers
const GRUVBOX_PURPLE: &str = "\x1b[38;5;175m"; // Types
const GRUVBOX_AQUA: &str = "\x1b[38;5;108m"; // Operators
const GRUVBOX_ORANGE: &str = "\x1b[38;5;208m"; // Built-in functions
const RESET: &str = "\x1b[0m";
impl<'a> Highlighter for PromptCompleter<'a> {
    fn highlight_char(&self, line: &str, pos: usize, kind: rustyline::highlight::CmdKind) -> bool {
        true
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        let mut result = String::new();
        let mut chars = line.chars().peekable();
        let mut current_word = String::new();

        while let Some(ch) = chars.next() {
            match ch {
                // dot commands
                '.' if current_word.is_empty() => {
                    result.push_str(GRUVBOX_ORANGE);
                    result.push(ch);
                    while let Some(&c) = chars.peek() {
                        if c == ' ' {
                            break;
                        }
                        chars.next();
                        result.push(c);
                    }
                    result.push_str(RESET);
                }
                // String literals
                '\'' => {
                    if !current_word.is_empty() {
                        result.push_str(&current_word);
                        current_word.clear();
                    }
                    result.push_str(GRUVBOX_GREEN);
                    result.push(ch);
                    while let Some(&next_ch) = chars.peek() {
                        chars.next();
                        result.push(next_ch);
                        if next_ch == '\'' {
                            break;
                        }
                        if next_ch == '\\' {
                            if let Some(&escaped) = chars.peek() {
                                chars.next();
                                result.push(escaped);
                            }
                        }
                    }
                    result.push_str(RESET);
                }
                // Numbers
                '0'..='9' if current_word.is_empty() => {
                    result.push_str(GRUVBOX_BLUE);
                    result.push(ch);
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch.is_numeric() || next_ch == '.' {
                            chars.next();
                            result.push(next_ch);
                        } else {
                            break;
                        }
                    }
                    result.push_str(RESET);
                }
                // Operators and punctuation
                '=' | '<' | '>' | '+' | '-' | '*' | '/' | '%' | '!' => {
                    if !current_word.is_empty() {
                        result.push_str(&current_word);
                        current_word.clear();
                    }
                    result.push_str(GRUVBOX_AQUA);
                    result.push(ch);
                    result.push_str(RESET);
                }
                // Word boundaries
                ' ' | ',' | ';' | '(' | ')' | '\t' | '\n' => {
                    if !current_word.is_empty() {
                        let upper = current_word.to_uppercase();
                        if consts::is_sqlite_keyword(&current_word) {
                            result.push_str(GRUVBOX_RED);
                            result.push_str(&upper);
                            result.push_str(RESET);
                        } else if consts::is_sqlite_type(&current_word) {
                            result.push_str(GRUVBOX_YELLOW);
                            result.push_str(&upper);
                            result.push_str(RESET);
                        } else {
                            result.push_str(&current_word);
                        }
                        current_word.clear();
                    }
                    result.push(ch);
                }
                // Build up word
                _ => {
                    current_word.push(ch);
                }
            }
        }

        // Handle remaining word
        if !current_word.is_empty() {
            let upper = current_word.to_uppercase();
            if consts::is_sqlite_keyword(&current_word) {
                result.push_str(GRUVBOX_RED);
                result.push_str(&upper);
                result.push_str(RESET);
            } else if consts::is_sqlite_type(&current_word) {
                result.push_str(GRUVBOX_YELLOW);
                result.push_str(&upper);
                result.push_str(RESET);
            } else {
                result.push_str(&current_word);
            }
        }

        Cow::Owned(result)
    }
}
impl<'a> Hinter for PromptCompleter<'a> {
    type Hint = &'static str;
}
impl<'a> Validator for PromptCompleter<'a> {}

pub struct Prompt<'a> {
    editor: Editor<PromptCompleter<'a>, FileHistory>,
    hist_file: PathBuf,
}
impl<'a> Prompt<'a> {
    pub fn new() -> Self {
        let editor_cfg = Config::builder()
            .edit_mode(EditMode::Vi)
            .color_mode(rustyline::ColorMode::Enabled)
            .build();
        let history_cfg = Config::builder()
            .history_ignore_space(true)
            .max_history_size(1024)
            .unwrap()
            .build();
        let file_history = FileHistory::with_config(&history_cfg);
        let mut editor =
            Editor::with_history(editor_cfg, file_history).expect("unable to create a prompt");

        let completer = PromptCompleter::new();
        editor.set_helper(Some(completer));
        editor.set_completion_type(CompletionType::List);

        let mut hist_file = std::env::home_dir().unwrap_or(PathBuf::from(".shqlite_history"));
        hist_file.push(".shqlite_history");

        Self { editor, hist_file }
    }

    pub fn save_history(&mut self) -> rustyline::Result<()> {
        self.editor.save_history(&self.hist_file)
    }

    pub fn add_history_entry(&mut self, entry: &str) -> rustyline::Result<bool> {
        self.editor.add_history_entry(entry)
    }

    pub fn readline(&mut self) -> rustyline::Result<String> {
        self.editor.readline("shqlite> ")
    }
}
