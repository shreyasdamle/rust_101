pub fn run() {
    let result = is_palindrome("taccat");
    println!("Result: {}", result);
}

fn is_palindrome(input: &str) -> bool {
    let rev_string: String = input.chars().rev().collect();
    if input == rev_string {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("taccat"), true);
    }
}
