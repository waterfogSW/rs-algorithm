use std::collections::HashMap;
use crate::Solution;

impl Solution {

    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            match map.get(&num) {
                Some(&j) => return vec![i as i32, j as i32],
                None => {
                    map.insert(target - num, i);
                    continue
                },
            }
        }
        unreachable!()
    }
}



impl Solution2 for Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, &num1) in nums.iter().enumerate() {
            for (j, &num2) in nums.iter().enumerate() {
                if i == j {
                    continue;
                }
                if num1 + num2 == target {
                    return vec![i as i32, j as i32]
                }
            }
        }
        vec![]
    }
}
