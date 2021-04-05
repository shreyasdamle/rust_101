//Data Types
/*
Rust is a statically typed language, which means that it must know the types of all variables at compile time,
however, the compiler can usually infer what type we want to use based on the value and how we use it.
*/

pub fn run() {
    //Scalar Types (Stored on the stack)

    //Integer (u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, isize, usize)
    // default is i32
    let x = 5;
    println!("The value of x: {}", x);
    // add explicit type
    let y: u16 = 32;
    println!("The value of y: {}", y);

    //Floating point (f32, f64)
    // default is f64
    let m = 5.5;
    println!("The value of m: {}", m);
    // add explicit type
    let n: f32 = 32.12;
    println!("The value of n: {}", n);

    //Boolean (bool)
    let t = true;
    println!("The value of t: {}", t);
    //add explicit type
    let f: bool = false;
    println!("The value of n: {}", f);

    //Character (char)
    let c = 's';
    let e = '😀';
    println!("The value of c: {} and e: {}", c, e);

    //Compound Types (Stored on the stack)

    //Tuple (diff type and fixed length)
    let tup = (2, 's', true);
    println!("The value of tup: {:?}", tup);
    //add explicit type
    let e_tup: (i32, char, f32, bool) = (5, 'c', 2.2, true);
    println!("The value of tup: {:?}", e_tup);
    //bind to variables
    let (a, b, c, d) = e_tup;
    println!("The value of a: {}, b:{}, c:{}, d:{}", a, b, c, d);
    //access tuple element
    let first_elemet = e_tup.0;
    println!("The value of first_elemet: {}", first_elemet);

    //Array (same type and fixed length)
    let arr = [1, 2, 3];
    println!("The value of tup: {:?}", arr);
    //add explicit type
    let e_arr: [char; 3] = ['x', 'y', 'z'];
    println!("The value of e_arr: {:?}", e_arr);
    //access array element
    let first = arr[0];
    println!("The value of first: {}", first);
}
