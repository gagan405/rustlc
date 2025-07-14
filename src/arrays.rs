use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, &e) in nums.iter().enumerate() {
        if let Some(&v)  = map.get(&(target - e)) {
            return vec![v, i as i32];
        }
        map.insert(e, i as i32);
    }
    vec![-1, -1]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_test() {
        let result = two_sum(vec![2, 6, 11, -2], 17);
        let expected = vec![1, 2];

        assert_eq!(result, expected);

        let result = two_sum(vec![2, 6, 11, -2], 10);
        let expected = vec![-1, -1];

        assert_eq!(result, expected);
    }
}
