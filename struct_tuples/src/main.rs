struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(1, 2, 3);
    let origin = Point(4, 5, 6);

    println!("black: Color({}, {}, {})", black.0, black.1, black.2);
    println!("origin: Point({}, {}, {})", origin.0, origin.1, origin.2);
}
