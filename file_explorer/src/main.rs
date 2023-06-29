mod dirs_and_files;
use dirs_and_files::{get_dir_entries, open_file};

mod menu;
use menu::Command;
use menu::Menu;

use std::path;

fn main() {
    let mut path = path::PathBuf::from("C:\\");
    let mut menu = Menu::new();

    loop {
        let dir_entries = get_dir_entries(&path);
        menu.display(path.to_str().unwrap(), dir_entries);

        match menu.get_command() {
            Command::Return => {
                path.pop();
            }
            Command::Open(item) => {
                path.push(item);
                if path.is_file() {
                    open_file(&mut path)
                }
            }
        }
    }
}
