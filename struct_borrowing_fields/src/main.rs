struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point{
        x: 0,
        y: 0,
    };
    // The same as:
    // let pointer: &mut Point = &mut p;
    let pointer = &mut p;

    // Change the reference
    pointer.x = 99;
    pointer.y = 99;

    println!("p = Point({}, {})!", p.x, p.y);
}
