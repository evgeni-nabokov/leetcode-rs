mod logger_v1;
mod logger_v2;
mod myhashset;

#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 520. Detect Capital.
    // https://leetcode.com/problems/detect-capital/
    pub fn detect_capital_use(word: String) -> bool {
        let mut is_upper_allowed = true;
        let mut is_lower_allowed = true;
        for (i, c) in word.chars().enumerate() {
            match (c.is_ascii_uppercase(), is_upper_allowed, is_lower_allowed) {
                (false, _, true) => { is_upper_allowed = false; },
                (true, false, _) | (false, _, false) => { return false; },
                (true, _, _ ) if i == 1 => { is_lower_allowed = false; }
                _ => (),
            }
        }
        true
    }

    // 125. Valid Palindrome.
    // https://leetcode.com/problems/valid-palindrome/
    pub fn is_palindrome(s: String) -> bool {
        if s.len() < 2 { return true; }
        let mut left = 0;
        let mut right = s.len() - 1;
        let chars: Vec<_> = s.chars().collect();
        loop {
            while left < right && !chars[left].is_alphanumeric() {
                left += 1;
            }
            while left < right && !chars[right].is_alphanumeric() {
                right -= 1;
            }
            if !chars[left].eq_ignore_ascii_case(&chars[right]) {
                return false;
            }
            if left >= right {
                return true;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}