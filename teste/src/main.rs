use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use std::thread;
use std::time::Duration;

fn main() {
    hide_cursor();
    loop {
        sleep(200);
        clear_terminal();
        move_cursor(20, 20);
        println!("oi");
    }
}

fn hide_cursor() {
    stdout()
        .execute(crossterm::cursor::Hide)
        .expect("Erro ao ocultar o cursor");
}

fn show_cursor() {
    stdout()
        .execute(crossterm::cursor::Show)
        .expect("Erro ao ocultar o cursor");
}

fn move_cursor(x: u16, y: u16) {
    stdout()
        .execute(MoveTo(x, y))
        .expect("Erro ao mover cursor");
}

fn clear_terminal() {
    stdout()
        .execute(Clear(ClearType::All))
        .expect("Erro ao limpar terminal");
}

fn sleep(miliseconds: u64) {
    thread::sleep(Duration::from_millis(miliseconds));
}
