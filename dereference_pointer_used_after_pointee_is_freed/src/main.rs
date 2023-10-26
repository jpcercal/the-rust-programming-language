fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4);
    println!("Third element is {}", *num); // undefined behavior: pointer used after its pointee is freed on the line before
}
