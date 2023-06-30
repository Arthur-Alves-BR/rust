mod interface;
mod snake;

use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::enable_raw_mode,
};
use interface::clear_terminal;

use crate::interface::move_cursor;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    interface::hide_cursor();
    interface::clear_terminal();

    let mut counter = 0;
    let mut direction = Direction::Right;
    let mut snake = snake::Snake::new(2, 2);

    let (sender, receiver) = channel::<Direction>();

    thread::spawn(move || get_user_input(sender));

    interface::draw_arena(100, 15);
    loop {
        if let Ok(event) = receiver.try_recv() {
            direction = event;
        }

        snake.draw();
        snake.erase();
        snake.move_head(&direction);

        sleep(20);
    }
}

fn get_user_input(sender: Sender<Direction>) {
    loop {
        let key_event = get_key_event();
        if key_event.kind == KeyEventKind::Release {
            continue;
        }
        _ = match key_event.code {
            KeyCode::Up => sender.send(Direction::Up),
            KeyCode::Down => sender.send(Direction::Down),
            KeyCode::Left => sender.send(Direction::Left),
            KeyCode::Right => sender.send(Direction::Right),
            _ => {
                continue;
            }
        };
    }
}

fn get_key_event() -> KeyEvent {
    loop {
        if let Ok(Event::Key(key_event)) = read() {
            break key_event;
        }
    }
}

fn sleep(miliseconds: u64) {
    thread::sleep(Duration::from_millis(miliseconds));
}
