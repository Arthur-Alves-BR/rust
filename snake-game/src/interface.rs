use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

pub fn draw_arena(width: u16, height: u16) {
    for i in 0..=width {
        for j in 0..=height {
            move_cursor(i, j);
            if i == 0 || i == width || j == 0 || j == height {
                print!("\u{2588}");
                stdout().flush().unwrap();
            }
        }
    }
    println!();
}

pub fn hide_cursor() {
    stdout()
        .execute(crossterm::cursor::Hide)
        .expect("Erro ao ocultar o cursor");
}

// pub fn show_cursor() {
//     stdout()
//         .execute(crossterm::cursor::Show)
//         .expect("Erro ao ocultar o cursor");
// }

pub fn move_cursor(x: u16, y: u16) {
    stdout()
        .execute(MoveTo(x, y))
        .expect("Erro ao mover cursor");
}

pub fn clear_terminal() {
    stdout()
        .execute(Clear(ClearType::All))
        .expect("Erro ao limpar terminal");
}
