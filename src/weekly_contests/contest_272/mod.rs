#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 2108. Find First Palindromic String in the Array.
    // https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn first_palindrome(words: Vec<String>) -> String {
        'outer: for w in words {
            let bytes = w.as_bytes();
            let mut left = 0;
            let mut right = bytes.len() - 1;
            while left < right {
                if bytes[left] != bytes[right] {
                    continue 'outer;
                }
                left += 1;
                right -= 1;
            }
            return w;
        }

        String::new()
    }
}
