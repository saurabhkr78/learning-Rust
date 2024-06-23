fn main() {
    let result = add(5, 3);
    println!("The result is: {}", result);
}
//returns an i32
fn add(x: i32, y: i32) -> i32 {
    x + y
}
