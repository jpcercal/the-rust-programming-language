#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // same as 
    // fn area(self: &Self) -> u32 { 
    // or 
    // fn area(self: &Rectangle) -> u32 {
    // or
    fn area(&self) -> u32 {
        // The &self is actually short for self: &Self. Within an impl block, the
        // type Self is an alias for the type that the impl block is for.
    
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}