struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels (using `area_with_primitives`).",
        area_with_primitives(width, height)
    );

    println!(
        "The area of the rectangle is {} square pixels (using `area_with_tuple`).",
        area_with_tuple((width, height))
    );

    println!(
        "The area of the rectangle is {} square pixels (using `area_with_struct`).",
        area_with_struct(&Rectangle{
            width,
            height,    
        })
    );
}

fn area_with_primitives(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}