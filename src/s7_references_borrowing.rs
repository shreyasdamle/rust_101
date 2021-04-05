// References and Borrowing
/*
At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.
*/
pub fn run() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); //pass reference

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    // change(&s1); (does not work)
    change(&mut s); //works

    let mut s2 = String::from("hello");
    let r1 = &mut s2;
    // let r2 = &mut s2; (cannot borrow `s` as mutable more than once at a time)
    println!("{}", r1);

    // Dangling References
    // let reference_to_nothing = dangle();

    let _reference_to_nothing = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len() //borrow reference (immutable)
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
