use std::ffi::OsString;
use std::fs::DirEntry;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Menu {
    options: Vec<String>,
}

impl Menu {
    pub const COME_BACK_MESSAGE: &str = "VOLTAR";

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
        self.options.push(Self::COME_BACK_MESSAGE.to_string());
    }

    pub fn display(&self, path: &str) {
        println!("--------------------------------------------------------");
        println!("{}", path);
        println!("--------------------------------------------------------");
        for (index, value) in self.options.iter().enumerate() {
            println!("{index} - {value}");
        }
    }

    pub fn get_user_choice(&self) -> String {
        self.options[self.get_user_input() as usize].clone()
    }

    fn get_user_input(&self) -> u32 {
        loop {
            print!("Escolha uma opção: ");
            io::stdout().flush().unwrap();

            let mut input: String = String::new();
            let error_str = "Valor inválido! Tente novamente...";

            io::stdin()
                .read_line(&mut input)
                .expect("Falha ao ler entrada");

            match input.trim().parse() {
                Ok(num) => {
                    if num <= (self.options.len() - 1) as u32 {
                        return num;
                    } else {
                        println!("{}", error_str);
                    }
                }
                Err(_) => {
                    println!("{}", error_str);
                }
            }
        }
    }

    fn clear(&mut self) {
        self.options.clear();
    }
}
