use std::collections::HashMap;

pub fn run() {
    let result = is_permutation("abc", "cba");
    println!("Result: {}", result);
}

//CTCI: Is permutation
fn is_permutation(input1: &str, input2: &str) -> bool {
    if input1.len() != input2.len() {
        return false;
    }
    let count_map_input1 = count_char(input1);
    let count_map_input2 = count_char(input2);

    if count_map_input1 == count_map_input2 {
        return true;
    }
    false
}

fn count_char(s: &str) -> HashMap<char, i32> {
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
    fn test_is_permutation() {
        assert_eq!(
            is_permutation(&String::from("asdf"), &String::from("dsaf")),
            true
        );
        assert_eq!(is_permutation("asdf", "safd"), true);
        assert_eq!(is_permutation("asdf", "zsdf"), false);
        assert_eq!(is_permutation("alex", "alet"), false);
    }
}
