mod rectangle;

pub use crate::rectangle::Rectangle;

fn main() {
    let r = Rectangle{width: 10, height: 10};
    let r2 = Rectangle{width: 9, height: 9};
    let r3 = Rectangle::square(5);
    println!("{}", r.area());
    println!("{}", r.can_hold(&r2));
    println!("{}", r.can_hold(&r3));
}
