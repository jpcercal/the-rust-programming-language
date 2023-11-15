fn main() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");
}
