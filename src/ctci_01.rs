use std::collections::HashSet;

pub fn run() {
    let result = all_chars_unique("Shreyas");
    println!("Result: {}", result);
}

//CTCI: Is unique
fn all_chars_unique(input: &str) -> bool {
    let mut char_set: HashSet<char> = HashSet::new();
    for c in input.chars() {
        if char_set.contains(&c) {
            return false;
        }
        char_set.insert(c);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert_eq!(all_chars_unique(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique(&String::from("abcdefga")), false);
    }
}
