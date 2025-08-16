use std::{io, ops::Add};
mod re;
mod store;
mod user;

struct Point<T, U> {
    x: T,
    y: U,
}

impl Point<i32, f64> {
    fn add(&self) -> i32 {
        self.x + self.y.floor() as i32
    }
}

impl<T> Point<T, f64> {
    fn fuck_off(&self) -> i32 {
        10
    }

    fn add(&self) -> i32 {
        10
    }
}

fn main() -> Result<(), io::Error> {
    let p = Point { x: 10, y: 20.5 };
    println!("{}", p.add());

    Ok(())
}
