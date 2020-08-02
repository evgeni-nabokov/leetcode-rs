mod logger_v1;
mod logger_v2;

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
}