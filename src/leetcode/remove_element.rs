struct Solution;

/****************************************************
    Problem: 27. Remove Element
    Website: https://leetcode.com/problems/remove-element/
    Difficulty: Easy
****************************************************
 */

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        /**
         * 불변성을 유지하면서 새로운 벡터를 생성하는 방법:
         * 1. `iter()` 메서드를 사용하여 `nums` 벡터의 불변 참조를 얻습니다.
         * 2. `filter()` 메서드를 사용하여 `val`과 같지 않은 요소만 선택합니다.
         *    - `|&x|`는 `&i32` 타입의 참조를 패턴 매칭을 통해 받아옵니다.
         *    - `*x != val`에서 `*`는 참조를 역참조하여 실제 값과 비교합니다.
         * 3. `map()` 메서드를 사용하여 선택된 요소를 `i32` 타입으로 변환합니다.
         *    - `|x|`는 `&i32` 타입의 참조를 받아옵니다.
         *    - `*x`는 참조를 역참조하여 `i32` 값을 반환합니다.
         * 4. `collect()` 메서드를 사용하여 새로운 벡터를 생성합니다.
         * 5. 생성된 새로운 벡터를 `nums`에 복사합니다.
         * 6. 새로운 벡터의 길이를 `i32` 타입으로 반환합니다.
         */
        // let mut result: Vec<i32> = nums.iter().filter(|&x| *x != val).map(|x| *x).collect();
        // for i in 0..result.len() {
        //     nums[i] = result[i];
        // }
        // result.len() as i32

        /**
         * `nums` 벡터를 직접 변경하는 방법:
         * 1. `retain()` 메서드를 사용하여 조건에 맞는 요소만 유지합니다.
         *    - `|&x|`는 `i32` 타입의 값을 참조를 통해 받아옵니다.
         *    - `x != val`에서 `x`는 이미 `i32` 값이므로 역참조가 필요하지 않습니다.
         * 2. `retain()` 메서드는 `nums` 벡터를 직접 변경합니다.
         * 3. 변경된 `nums` 벡터의 길이를 `i32` 타입으로 반환합니다.
         */
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}
