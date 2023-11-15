fn main() {
    // However, this undefined behavior does not happen when the vector contains i32 elements. The difference is that copying a String copies a pointer to heap data. Copying an i32 does not. In technical terms, Rust says that the type i32 implements the Copy trait, while String does not implement Copy (we will discuss traits in a later chapter).
    // In sum, if a value does not own heap data, then it can be copied without a move. For example:
    // An i32 does not own heap data, so it can be copied without a move.
    // A String does own heap data, so it can not be copied without a move.
    // An &String does not own heap data, so it can be copied without a move.
    let v: Vec<i32> = 
    vec![111111111];
    let s_ref: &i32 = &v[0];
    let s: i32 = *s_ref;

    println!("here goes the number assigned to s {}", s);

    // These drops are normally implicit, but we've added them for clarity.
    drop(s);
    drop(v);
}
