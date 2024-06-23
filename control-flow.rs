fn main() {
    let number = 5;

    if number < 10 {
        println!("The number is less than 10");
    } else {
        println!("The number is 10 or greater");
    }

    for i in 1..5 {
        println!("i: {}", i);
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);
}
