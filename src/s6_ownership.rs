// Ownership
/*
Ownership rules:
Each value in Rust has a variable that’s called its owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
*/
pub fn run() {
    // string literal (immutable)
    let sl = "World!";

    // String type (mutable; stored on heap)
    let mut sm = String::from("Hello, ");
    sm.push_str(sl);
    println!("{}", sm);

    // this works
    let test_sl = sl;
    println!("{}, {}", sl, test_sl);

    // this doesn't work (Rust will never automatically create “deep” copies)
    // let test_sm = sm;
    // println!("{}, {}", sm, test_sm);

    // if deep copy required, use clone
    let clone_sm = sm.clone();
    println!("{}, {}", sm, clone_sm);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // println!("{}", s); // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
    println!("{}", x); // but i32 is Copy, so it’s okay to still
                       // use x afterward

    // Returning values can also transfer ownership
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("{}", s1);
    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
                                       // println!("{}", s2);
    println!("{}", s3);

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

/*
Taking ownership and then returning ownership with every function is a bit tedious.
What if we want to let a function use a value but not take ownership?
It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again,
in addition to any data resulting from the body of the function that we might want to return as well.
It’s possible to return multiple values using a tuple
*/
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
