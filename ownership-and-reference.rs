fn main() {
    /*Each value in Rust has a single owner,
     and when the owner goes out of scope, the value is dropped. */
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 is no longer valid here, as ownership has moved to s2
    println!("s2: {}", s2);

    let s3 = String::from("world");
    let s4 = &s3; // s4 is a reference to s3, so s3 is still valid
    println!("s3: {} and s4: {}", s3, s4);
}
