fn main() {
    let v: Vec<String> = 
    vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    let s: String = *s_ref;

    // These drops are normally implicit, but we've added them for clarity.
    drop(s);
    drop(v);
}
