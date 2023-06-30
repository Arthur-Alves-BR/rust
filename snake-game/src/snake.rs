use crate::{interface, Direction};
use std::io::{stdout, Write};

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug)]
struct SnakePart {
    symbol: char,
    position: Point,
}

#[derive(Debug)]
pub struct Snake {
    parts: Vec<SnakePart>,
}

impl SnakePart {
    fn draw(&self) {
        interface::move_cursor(self.position.x, self.position.y);
        print!("{}", self.symbol);
        stdout().flush().unwrap();
    }

    fn erase(&self) {
        interface::move_cursor(self.position.x + 1, self.position.y);
        print!("\u{8} \u{8}");
    }
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            parts: Vec::from([SnakePart {
                symbol: '\u{1F62E}',
                // symbol: 'X',
                position: Point { x, y },
            }]),
        }
    }

    pub fn move_head(&mut self, direction: &Direction) {
        let head = &mut self.parts[0];
        match direction {
            Direction::Up => {
                head.position.y -= 1;
            }
            Direction::Down => {
                head.position.y += 1;
            }
            Direction::Left => {
                head.position.x -= 1;
            }
            Direction::Right => {
                head.position.x += 1;
            }
        }
    }

    pub fn draw(&self) {
        for part in self.parts.iter() {
            part.draw();
        }
    }

    pub fn erase(&self) {
        self.parts.last().unwrap().erase();
        // for part in self.parts.iter() {
        //     part.erase();
        // }
    }

    // pub fn grow(&mut self) {
    //     self.parts.push(SnakePart {
    //         symbol: 'U',
    //         position: Point {
    //             x: self.head.position.x + 1,
    //             y: self.head.position.y + 1,
    //         },
    //     })
    // }
}
