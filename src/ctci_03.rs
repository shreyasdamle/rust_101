pub fn run() {
    let result = urlify("abc def");
    println!("Result: {}", result);
}

// CTCI Urlify
fn urlify(input: &str) -> String {
    input.trim().replace(" ", "%20")
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_urlify() {
        assert_eq!(urlify("abc def"), "abc%20def");
    }
}
