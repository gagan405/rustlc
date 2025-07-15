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

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mut start = 0;
    let mut end = nums.len() as isize -1;

    while start <= end {
        let mid = start + (end - start) / 2;
        let mid_val = nums[mid as usize];

        if mid_val == target {
            return mid as i32;
        }
        if target > mid_val {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_palindrome_test() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(122), false);
    }

    #[test]
    fn test_search() {
        assert_eq!(search(vec![1, 3, 4, 7, 8], 4), 2);
        assert_eq!(search(vec![1, 3, 4, 7, 8], 5), -1);
        assert_eq!(search(vec![5], -5), -1);
    }
}