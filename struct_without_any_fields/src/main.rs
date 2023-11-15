#[derive(Debug)]
struct EmptyStruct;

#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
    let empty_struct = EmptyStruct;

    println!("{:?}", subject);
    println!("{:?}", empty_struct);
}
