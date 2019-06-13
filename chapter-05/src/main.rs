#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rect {
        width: 12,
        height: 34,
    };
    println!("the rect: {:?}, the area: {}", rect, area(&rect));
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
