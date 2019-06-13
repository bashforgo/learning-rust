#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn grow(&mut self) {
        self.width += 1;
        self.height += 1;
    }

    fn invalidate(self) {}
}

fn main() {
    let mut rect = Rect {
        width: 12,
        height: 34,
    };
    println!("the rect: {:?}, the area: {}", rect, rect.area());
    rect.grow();
    println!("new rect: {:?}, area: {}", rect, rect.area());
    rect.invalidate();
    // println!("new rect: {:?}, area: {}", rect, rect.area());

    let r1 = Rect {
        width: 10,
        height: 10,
    };
    let r2 = Rect {
        width: 100,
        height: 100,
    };
    let r3 = Rect {
        width: 5,
        height: 60,
    };
    println!("r1 can hold r2: {}", r1.can_hold(&r2));
    println!("r1 can hold r3: {}", r1.can_hold(&r3));
    println!("r2 can hold r1: {}", r2.can_hold(&r1));
    println!("r2 can hold r3: {}", r2.can_hold(&r3));
    println!("r3 can hold r1: {}", r3.can_hold(&r1));
    println!("r3 can hold r2: {}", r3.can_hold(&r2));
}
