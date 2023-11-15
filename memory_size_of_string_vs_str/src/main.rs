fn main() {
    // Context: The type &String is a normal reference consisting of a single
    // pointer, so 8 bytes on a 64-bit architecture. The type &str is a special
    // slice reference which consists of a pointer and a length, so 16 bytes.
    // Therefore s3 of type &str uses more memory than s2 of type &String.
    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}
