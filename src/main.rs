#[derive(Clone, Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: Rect) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 31,
        height: 40,
    };

    let rect3 = Rect::square(10);
    println!("rect3: {rect3:#?}");

    println!("area1: {}", rect1.area());
    println!("area2: {}", rect2.area());

    println!("Can rect1 hold rect2: {}", rect1.can_hold(rect2));
}
