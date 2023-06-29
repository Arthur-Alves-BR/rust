use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::{self, Write};

fn main() {
    println!("{:?}", get_input_message());
}

fn get_input_message() -> String {
    let mut message = String::new();

    loop {
        let key_event = get_key_event();
        if key_event.kind == KeyEventKind::Release {
            continue;
        }
        match key_event.code {
            KeyCode::Char(ch) => {
                message.push(ch);
                print_char(ch);
            }
            KeyCode::Backspace => {
                message.pop();
                delete_char();
            }
            KeyCode::Enter => break,
            _ => {}
        }
    }
    println!();

    message
}

fn get_key_event() -> KeyEvent {
    loop {
        if let Ok(Event::Key(key_event)) = read() {
            break key_event;
        }
    }
}

fn print_char(ch: char) {
    print!("{}", ch);
    io::stdout().flush().unwrap();
}

fn delete_char() {
    print!("\u{8} \u{8}");
    io::stdout().flush().unwrap();
}
