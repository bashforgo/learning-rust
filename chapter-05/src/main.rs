#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
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
}
