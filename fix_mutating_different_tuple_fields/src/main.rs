fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn main() {
    let mut name = (
        String::from("Ferris"), 
        String::from("Rustacean")
    );
    let first = get_first(&name);
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}
