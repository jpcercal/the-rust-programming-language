fn main() {
    let mut s = String::from("hello");
    let hello: &str = &s[0..5];
    println!("{hello}");
    s.push_str(" world");
    println!("{s}");
}
