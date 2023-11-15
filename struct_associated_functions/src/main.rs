#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle::square(50);
    let rect2 = Rectangle::square(40);
    let rect3 = Rectangle::square(60);

    println!("rect1 = {:?}", rect1);
    println!("rect2 = {:?}", rect2);
    println!("rect3 = {:?}", rect3);
}
