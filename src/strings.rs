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

pub fn roman_to_int(s: String) -> i32 {
    if s.is_empty() { return 0; }

    let map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut num = *map.get(&s.chars().nth(s.len() - 1).unwrap()).unwrap();

    for i in (0..s.len() - 1).rev() {
        let curr = map.get(&s.chars().nth(i).unwrap()).unwrap();
        let prev = map.get(&s.chars().nth(i + 1).unwrap()).unwrap();

        if curr < prev {
            num = num - curr;
        } else {
            num = num + curr;
        }
    }
    num
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    fn all_same_char(idx: usize, strs: &Vec<String>) -> bool {
        if idx >= strs[0].len() {
            return false;
        }
        let c = strs[0].as_bytes()[idx];
        for s in strs.iter().skip(1) {
            if idx >= s.len() || s.as_bytes()[idx] != c {
                return false;
            }
        }
        true
    }

    let mut idx = 0;
    while all_same_char(idx, &strs) {
        idx += 1;
    }
    strs[0][0..idx].to_string()
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

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int(String::from("IV")), 4);
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(longest_common_prefix(vec![]), "".to_string());
        assert_eq!(longest_common_prefix(vec![String::from("i")]), "i".to_string());
        assert_eq!(longest_common_prefix(vec![String::from("i"), String::from("i")]), "i".to_string());
        assert_eq!(longest_common_prefix(vec![String::from("ab"), String::from("ab"), String::from("abc")]), "ab".to_string());
    }
}