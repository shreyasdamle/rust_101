// Variables and Mutability
// By default variables are immutable

pub fn run() {
    // immutable var
    let x = 5;
    println!("The value of x: {}", x);

    //mutable var
    let mut y = 10;
    println!("The value of y: {}", y);
    y = y * 2;
    println!("The value of y: {}", y);

    //const are always mutable and type of the value must be annotated
    const MAX_POINTS: i32 = 100;
    println!("The value of MAX_POINTS: {}", MAX_POINTS);

    //shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z: {}", z);
}
