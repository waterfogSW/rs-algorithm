struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let mut i = 0;
        let mut j = 0;

        while i < haystack.len() {
            if haystack[i] == needle[j] {
                j += 1;
                if j == needle.len() {
                    return (i - j + 1) as i32;
                }
            } else {
                i -= j;
                j = 0;
            }
            i += 1;
        }

        -1
    }
}



#[test]
fn test() {
    let result = Solution::str_str("mississippi".to_string(), "issipi".to_string());
    assert_eq!(result, -1);
}
