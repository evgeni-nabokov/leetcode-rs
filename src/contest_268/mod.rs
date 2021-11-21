#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 2078. Two Furthest Houses With Different Colors.
    // https://leetcode.com/problems/two-furthest-houses-with-different-colors/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let end = colors.len() - 1;

        if colors[0] != colors[end] {
            return end as i32;
        }

        let mut left = 1;
        while colors[left] == colors[0] {
            left += 1;
        }

        let mut right = end - 1;
        while colors[right] == colors[end] {
            right -= 1;
        }

        right.max(end - left) as i32
    }
}