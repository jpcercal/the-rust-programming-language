fn main() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
                            //     so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;    // so only one dereference is needed to read it

    println!("x: {:?}", x);
    println!("a: {:?}", a);

    println!("r1: {:?}", r1);
    println!("b: {:?}", b);

    println!("r2: {:?}", r2);
    println!("c: {:?}", c);

    let xbox = Box::new(0);
    let ybox = Box::new(&xbox);

    println!("xbox: {:?}", xbox);
    println!("ybox: {:?}", ybox);
}
