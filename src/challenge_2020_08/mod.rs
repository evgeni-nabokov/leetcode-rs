mod logger_v1;
mod logger_v2;
mod hashset;

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

    // 342. Power of Four.
    // https://leetcode.com/problems/power-of-four/
    pub fn is_power_of_four(mut num: i32) -> bool {
        if num == 1 { return true; }
        if num < 4 || num & (num - 1) != 0 { return false; }
        let mut count = 0;
        while num > 1 {
            num >>= 1;
            count += 1;
        }
        count % 2 == 0
    }

    pub fn is_power_of_four_v2(num: i32) -> bool {
        let is_power_of_two = num > 0 && num & (-num) == num;
        is_power_of_two && (num & 0b1010101010101010101010101010101 != 0)
    }

    pub fn is_power_of_four_v3(num: i32) -> bool {
        let is_power_of_two = num > 0 && num & (-num) == num;
        is_power_of_two && num % 3 == 1
    }
}