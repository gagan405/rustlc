pub fn is_palindrome(number: i32) -> bool {
    if number < 0 { return false }

    let mut divisor = 1;
    while divisor <= number / 10 {
        divisor *= 10;
    }

    let mut x = number;
    loop {
        let right = x % 10;
        let left = x / divisor;
        if left != right {
            return false
        }

        x = (x % divisor) / 10;
        divisor = divisor / 100;

        if x == 0 {
            break;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_palindrome_test() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(122), false);
    }
}