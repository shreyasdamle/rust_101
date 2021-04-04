// Functions
pub fn run() {
    //Function within function
    fn greetings() {
        println!("Hello, there!");
    }

    greetings();
    no_param();

    with_param(5, true);

    with_exp();

    // Bind function values to variables
    let get_sum = add(8, 6);
    println!("The value of get_sum: {}", get_sum);

    let get_diff = sub(8, 6);
    println!("The value of get_diff: {}", get_diff);
}

//Function without params
fn no_param() {
    println!("Hello from no_param function!");
}

//Function with params
fn with_param(x: i32, y: bool) {
    println!("The value of x: {} and y: {}", x, y);
}

//Function with expression
fn with_exp() {
    let x = 5;

    //Expression
    let y = {
        let x = 3;
        x * 2
    };
    println!("The value of x*y: {}", x * y);
}

// Function with explicit return value (add two numbers)
fn add(x1: i32, x2: i32) -> i32 {
    return x1 + x2;
}

// Function implicit last line return
fn sub(n1: i32, n2: i32) -> i32 {
    n1 - n2
}
