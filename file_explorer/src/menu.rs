use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};

use std::ffi::OsString;
use std::fs::DirEntry;
use std::io::{self, Write};

pub enum Command {
    Open(String),
    Return,
}

#[derive(Debug)]
pub struct Menu {
    options: Vec<String>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
        }
    }

    pub fn display(&mut self, path: &str, dir_entries: Vec<DirEntry>) {
        println!();
        println!("--------------------------------------------------------");
        println!("{}", path);
        println!("--------------------------------------------------------");
        self.build_options(dir_entries);
        for (index, value) in self.options.iter().enumerate() {
            println!("{index} - {value}");
        }
    }

    pub fn get_command(&self) -> Command {
        self.flushed_print("Escolha uma opção (ou pressione TAB para retornar): ");

        let mut index_string = String::new();
        loop {
            let key_event = Menu::get_key_event();
            if key_event.kind == KeyEventKind::Release {
                continue;
            }
            match key_event.code {
                KeyCode::Char(ch) => {
                    if !ch.is_numeric() {
                        continue;
                    }
                    index_string.push(ch);
                    self.print_char(ch);
                }
                KeyCode::Backspace => {
                    index_string.pop();
                    self.delete_char();
                }
                KeyCode::Tab => {
                    return Command::Return;
                }
                KeyCode::Enter => {
                    let index = index_string.parse().ok();
                    if !self.is_valid_index(index) {
                        continue;
                    }
                    return Command::Open(self.get_user_choice(index.unwrap()));
                }
                _ => {}
            }
        }
    }

    fn is_valid_index(&self, index: Option<usize>) -> bool {
        match index {
            Some(i) => {
                if !(i < self.options.len()) {
                    return false;
                }
                true
            }
            None => false,
        }
    }

    fn build_options(&mut self, dir_entries: Vec<DirEntry>) {
        self.options.clear();
        self.options = dir_entries
            .iter()
            .map(DirEntry::file_name)
            .map(OsString::into_string)
            .map(Result::unwrap)
            .collect();
    }

    fn get_key_event() -> KeyEvent {
        loop {
            if let Ok(Event::Key(key_event)) = read() {
                break key_event;
            }
        }
    }

    fn get_user_choice(&self, index: usize) -> String {
        self.options[index].clone()
    }

    fn print_char(&self, ch: char) {
        self.flushed_print(&format!("{}", ch));
    }

    fn delete_char(&self) {
        self.flushed_print("\u{8} \u{8}");
    }

    fn flushed_print(&self, text: &str) {
        print!("{}", text);
        io::stdout().flush().unwrap();
    }
}
