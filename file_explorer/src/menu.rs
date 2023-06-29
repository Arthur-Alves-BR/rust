use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};

use std::ffi::OsString;
use std::fs::DirEntry;
use std::io::{self, Write};

pub enum Command {
    Open,
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

    pub fn build_options(&mut self, dir_entries: Vec<DirEntry>) {
        self.clear();
        self.options = dir_entries
            .iter()
            .map(DirEntry::file_name)
            .map(OsString::into_string)
            .map(Result::unwrap)
            .collect();
    }

    pub fn display(&self, path: &str) {
        println!();
        println!("--------------------------------------------------------");
        println!("{}", path);
        println!("--------------------------------------------------------");
        for (index, value) in self.options.iter().enumerate() {
            println!("{index} - {value}");
        }
    }

    pub fn get_user_choice(&self, index: usize) -> String {
        self.options[index].clone()
    }

    fn clear(&mut self) {
        self.options.clear();
    }

    pub fn get_command(&self) -> (Command, Option<usize>) {
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
                    return (Command::Return, None);
                }
                KeyCode::Enter => {
                    if index_string.is_empty() {
                        continue;
                    }
                    return (Command::Open, Some(index_string.parse().unwrap()));
                }
                _ => {}
            }
        }
    }

    pub fn get_key_event() -> KeyEvent {
        loop {
            if let Ok(Event::Key(key_event)) = read() {
                break key_event;
            }
        }
    }

    pub fn print_char(&self, ch: char) {
        self.flushed_print(&format!("{}", ch));
    }

    pub fn delete_char(&self) {
        self.flushed_print("\u{8} \u{8}");
    }

    pub fn flushed_print(&self, text: &str) {
        print!("{}", text);
        io::stdout().flush().unwrap();
    }
}
