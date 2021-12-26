#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 2119. A Number After a Double Reversal.
    // https://leetcode.com/problems/a-number-after-a-double-reversal/
    // Time complexity: O(1).
    // Space complexity: O(1).
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 != 0
    }
}
