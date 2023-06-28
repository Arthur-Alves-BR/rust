mod dirs_and_files;
use dirs_and_files::{get_dir_entries, open_file};

mod menu;
use menu::Menu;

use std::path;

fn main() {
    let mut path = path::PathBuf::from("C:\\");
    let mut menu = Menu::new();

    loop {
        let dir_entries = get_dir_entries(&path);

        menu.build_options(dir_entries);
        menu.display(path.to_str().unwrap());

        let user_choice = menu.get_user_choice();
        if user_choice == Menu::COME_BACK_MESSAGE {
            path.pop();
        } else {
            path.push(user_choice);
        }

        if path.is_file() {
            open_file(&mut path)
        }
    }
}
