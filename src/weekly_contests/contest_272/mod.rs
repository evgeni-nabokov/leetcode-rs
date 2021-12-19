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

    // 2109. Adding Spaces to a String.
    // https://leetcode.com/problems/adding-spaces-to-a-string/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let bytes = s.as_bytes();
        let mut spaced_bytes = vec![];
        let mut space_idx = 0;
        for i in 0..bytes.len() {
            if space_idx < spaces.len() && i == spaces[space_idx] as usize {
                spaced_bytes.push(b' ');
                space_idx += 1;
            }
            spaced_bytes.push(bytes[i]);
        }

        spaced_bytes.into_iter().map(|x| x as char).collect()
    }
}
