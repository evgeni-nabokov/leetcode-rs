#[cfg(test)]
mod tests;

use std::cmp::Ordering;

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

    // 2110. Number of Smooth Descent Periods of a Stock.
    // https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut res = 1;
        let mut sdp_len = 1;

        for i in 1..prices.len() {
            if prices[i - 1] - prices[i] == 1 {
                res += sdp_len;
                sdp_len += 1;
            } else {
                sdp_len = 1;
            }
            res += 1;
        }

        res
    }

    // 2111. Minimum Operations to Make the Array K-Increasing.
    // https://leetcode.com/problems/minimum-operations-to-make-the-array-k-increasing/
    // Time complexity: O(N * LogN). ?
    // Space complexity: O(N).
    pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let mut res = 0;
        let mut sub = vec![];
        for i in 0..k {
            sub.push(arr[i]);
            let mut j = i + k;
            let mut cnt = 1;
            while j < arr.len() {
                match arr[j].cmp(&sub[sub.len() - 1]) {
                    Ordering::Greater | Ordering::Equal => sub.push(arr[j]),
                    Ordering::Less => {
                        let mut left = 0;
                        loop {
                            match sub[left..].binary_search(&arr[j]) {
                                Err(x) => {
                                    sub[left + x] = arr[j];
                                    break;
                                }
                                Ok(x) => left = x + 1,
                            }
                        }
                    }
                }
                j += k;
                cnt += 1;
            }
            res += cnt - sub.len();
            sub.clear();
        }

        res as _
    }
}
