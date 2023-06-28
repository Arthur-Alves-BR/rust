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
    return match read_dir(path) {
        Ok(entries) => entries.filter(Result::is_ok).map(Result::unwrap).collect(),
        Err(_) => Vec::new(),
    };
}

pub fn get_all_dir_entries_below_path(path: &PathBuf) -> Vec<DirEntry> {
    let mut results: Vec<DirEntry> = Vec::new();
    let entries = get_dir_entries(path);

    for entry in &entries {
        if entry.path().is_file() {
            continue;
        }
        results.extend(get_all_dir_entries_below_path(&entry.path()));
    }

    results.extend(entries);
    results
}

pub fn search_in_dir(path: &PathBuf, expression: &String) -> Vec<DirEntry> {
    let mut results: Vec<DirEntry> = Vec::new();
    for entry in get_all_dir_entries_below_path(path) {
        if entry
            .file_name()
            .into_string()
            .unwrap()
            .contains(expression)
        {
            results.push(entry)
        }
    }
    results
}
