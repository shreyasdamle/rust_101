// Slice
pub fn run() {
    let s = String::from("hello, world");
    let hello = &s[0..5];
    println!("{}", hello);
    let slice = &s[0..2]; // or let slice = &s[..2];
    println!("{}", slice);
    let slice = &s[3..s.len()]; // or let slice = &s[3..];
    println!("{}", slice);
    let slice = &s[0..s.len()]; // or let slice = &s[..];
    println!("{}", slice);

    let sl = "String";
    let slice = &sl[0..2]; // or let slice = &s[..2];
    println!("{}", slice);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}
