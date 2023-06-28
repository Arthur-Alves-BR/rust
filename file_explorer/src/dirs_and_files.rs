use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;
use std::process::Command;

pub fn open_file(path: &mut PathBuf) {
    let _a = Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg(path.to_str().unwrap())
        .output()
        .expect("Falha ao abrir arquivo");
    path.pop();
}

pub fn get_dir_entries(path: &PathBuf) -> Vec<DirEntry> {
    read_dir(path)
        .unwrap()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect()
}
