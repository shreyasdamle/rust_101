// Control Flow

pub fn run() {
    // if expression
    let number = 24;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, and 2");
    }

    // use if in a let statement
    let condition = true;
    // each arm of the if must be of same type
    let number = if condition { 5 } else { 6 };
    println!("The value of number: {}", number);

    // loop - forever
    loop {
        println!("Hello from loop!");
        break;
    }

    // return values from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result: {}", result);

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop
    let a = [5, 4, 3, 2, 1];

    for element in a.iter() {
        println!("The value of element: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
