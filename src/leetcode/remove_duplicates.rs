use std::collections::HashSet;
use std::i32;

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let mut result = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            if !set.contains(&nums[i]) {
                set.insert(nums[i]);
                result.push(nums[i]);
            }
        }

        for i in 0..result.len() {
            nums[i] = result[i];
        }

        result.len() as i32
    }
}
