mod interface;
mod snake;

use std::thread;
use std::time::Duration;

fn main() {
    interface::clear_terminal();
    interface::draw_arena(100, 15);

    let snake = snake::Snake::new(2, 2);
    println!("{:?}", snake);

    snake.draw();

    sleep(2);
    interface::clear_terminal();
}

fn sleep(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}
