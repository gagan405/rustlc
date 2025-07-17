use std::collections::HashMap;

pub fn is_valid_parens(s: String) -> bool {
    let map: HashMap<char, char> = HashMap::from([
        (')', '('),
        ('}', '{'),
        (']', '['),
    ]);

    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        if let Some(&expected_open) = map.get(&c) {
            if stack.pop() != Some(expected_open) {
                return false;
            }
        } else {
            stack.push(c);
        }
    }
    stack.is_empty()
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.len() < needle.len() { return -1; }
    for i in 0..=haystack.len() - needle.len() {
        if haystack[i..i + needle.len()] == needle {
            return i as i32;
        }
    }
    -1
}

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for i in 1..=n {
        let divisible_by_fizz = i % 3 == 0;
        let divisible_by_buzz = i % 5 == 0;

        let mut str = String::new();

        if divisible_by_fizz { str.push_str("Fizz"); }
        if divisible_by_buzz { str.push_str("Buzz"); }

        if !divisible_by_buzz && !divisible_by_fizz {
            res.push(i.to_string());
        } else {
            res.push(str.to_string());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid_parens("()".to_string()), true);
        assert_eq!(is_valid_parens("[()]{}".to_string()), true);
        assert_eq!(is_valid_parens("[()]}{".to_string()), false);
        assert_eq!(is_valid_parens("}".to_string()), false);
    }

    #[test]
    fn test_str_str() {
        assert_eq!(str_str("a".to_string(), String::from("a")), 0);
        assert_eq!(str_str("sadbutsad".to_string(), String::from("sad")), 0);
        assert_eq!(str_str("sadbutsad".to_string(), String::from("bakchod")), -1);
    }

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(fizz_buzz(15), vec!["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]);
    }
}