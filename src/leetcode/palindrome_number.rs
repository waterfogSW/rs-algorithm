use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }

        let target_number = x;
        let mut x = x;
        let mut reversed = 0;
        while true {
            if x == 0 {
                break;
            }

            let remain = x % 10;
            reversed *= 10;
            reversed += remain;
            x /= 10;
        }


        return reversed == target_number
    }
}
//
// impl Solution2 for Solution {
//     fn is_palindrome(x: i32) -> bool {
//         return x.to_string().chars().rev().eq(x.to_string().chars());
//     }
// }
