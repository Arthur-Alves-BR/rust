use crate::interface::move_cursor;
use std::io::{stdout, Write};

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Head {
    symbol: char,
    position: Point,
}

impl Head {
    pub fn draw(&self) {
        move_cursor(self.position.x, self.position.y);
        print!("{}", self.symbol);
        stdout().flush().unwrap();
    }
}

#[derive(Debug)]
pub struct Snake {
    head: Head,
    parts: Vec<Point>,
    parts_symbol: char,
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            head: Head {
                symbol: 'X',
                position: Point::new(x, y),
            },
            parts: Vec::new(),
            parts_symbol: 'U',
        }
    }

    pub fn draw(&self) {
        self.head.draw();
    }
}
