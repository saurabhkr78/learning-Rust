enum Message {//enum named message
    Quit,//quit and move are variants that holds data  
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    //Creating an Enum Instance:
    let msg = Message::Move { x: 10, y: 20 };
   //Pattern Matching:'Match' is powerful control flow construct that allows you to compare a value against a series of patterns and execute code based on which pattern matches. It is somewhat similar to a switch statement in other languages but much more powerful and flexible.
    match msg {
        //: If msg matches the Quit variant, it executes println!("Quit")
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to red: {}, green: {}, blue: {}", r, g, b),
        _ => println!("Write: {}", text), //default: This matches any value not already matched

    }

}
//Destructuring: Allows extracting values from complex data types, such as enums, tuples, and structs.