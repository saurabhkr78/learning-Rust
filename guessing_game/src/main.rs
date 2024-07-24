use std::io;

fn main() {
    println!("Guess the Number");
    println!("Please!,Enter a guess:");
    // let statement, which is used to create a variable. 
    //In Rust, variables are immutable by default
    /*The :: syntax in the ::new line indicates that new is an associated function
of the String type. An associated function is implemented on a type, in this
case String, rather than on a particular instance of a String. Some languages
call this a static method.
This new function creates a new, empty string. You’ll find a new function
on many types, because it’s a co */
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("You guessed:{}",guess);//{} is a placeholder:You can print more than one value using curly brackets
}
