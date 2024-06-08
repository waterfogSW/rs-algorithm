use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            .reduce(|acc, cur| {
                acc.chars()
                    .zip(cur.chars())
                    .take_while(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect()
            })
            .unwrap()
    }

    // pub fn longest_common_prefix(strs: Vec<String>) -> String {
    //
    //
    //     if strs.is_empty() {
    //         return String::new();
    //     }
    //
    //     let mut shortest_str: String = String::new();
    //     let mut min_len: usize = usize::MAX;
    //
    //     for s in &strs {
    //         if s.len() < min_len {
    //             min_len = s.len();
    //             shortest_str = s.clone();
    //         }
    //     }
    //
    //     for i in 0..min_len {
    //         for s in &strs {
    //             if s.chars().nth(i).unwrap() != shortest_str.chars().nth(i).unwrap() {
    //                 return shortest_str[0..i].to_string();
    //             }
    //         }
    //     }
    //
    //     return shortest_str;
    // }
}
