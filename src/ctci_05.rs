use std::collections::HashMap;
pub fn run() {
    let result = is_palindrome_permutation("race car");
    println!("Result: {}", result);
}

fn is_palindrome_permutation(input: &str) -> bool {
    let string = input.to_lowercase().replace(" ", "");
    let char_map = count_chars(&string);
    let mut has_odd = false;

    for v in char_map.values() {
        if v % 2 != 0 {
            if has_odd {
                return false;
            }
            has_odd = true;
        }
    }
    true
}

fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        if let Some(count) = map.get_mut(&c) {
            *count += 1;
        } else {
            map.insert(c, 1);
        }
    }
    map
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_permutation() {
        assert_eq!(is_palindrome_permutation("Tact Coa"), true);
        assert_eq!(is_palindrome_permutation("Tact Ca"), true);
        assert_eq!(is_palindrome_permutation("Mr waldos jackso"), false);
    }
}
