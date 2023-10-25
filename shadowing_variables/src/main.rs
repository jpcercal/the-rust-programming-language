fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x + 10;
        println!("The inner value of x is: {x}");
    }

    println!("The value of x is: {x}");
}
