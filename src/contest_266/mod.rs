#[cfg(test)]
mod tests;

use std::collections::HashSet;

struct Solution;

impl Solution {
    // 2062. Count Vowel Substrings of a String.
    // https://leetcode.com/problems/count-vowel-substrings-of-a-string/
    // Time complexity: O(N^2).
    // Space complexity: O(1).
    pub fn count_vowel_substrings(word: String) -> i32 {
        if word.len() < 5 {
            return 0;
        }

        let vowels: [u8; 5] = [97, 101, 105, 111, 117]; // ['a', 'e', 'i', 'o', 'u']
        let bytes = word.as_bytes();
        let mut set = HashSet::with_capacity(5);
        let mut res = 0;
        for left in 0..=bytes.len() - 5 {
            set.clear();
            for right in left..bytes.len() {
                if vowels.contains(&bytes[right]) {
                    set.insert(bytes[right]);
                } else {
                    break;
                }
                if set.len() == 5 {
                    res += 1;
                }
            }
        }

        res
    }
}