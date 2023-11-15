fn main() {
    let s = String::from("hello");
    let len = s.len();
    println!("s = {s}");
    println!("len = {len}");

    // With Rust’s .. range syntax, if you want to start at index zero, you can drop the value before the two periods. In other words, these are equal:
    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    println!("slice1 = {slice1}");
    println!("slice2 = {slice2}");

    // By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:
    let slice3 = &s[3..len];
    let slice4 = &s[3..];
    
    println!("slice3 = {slice3}");
    println!("slice4 = {slice4}");

    // You can also drop both values to take a slice of the entire string. So these are equal:
    let slice5 = &s[0..len];
    let slice6 = &s[..];
    
    println!("slice5 = {slice5}");
    println!("slice6 = {slice6}");
}
 