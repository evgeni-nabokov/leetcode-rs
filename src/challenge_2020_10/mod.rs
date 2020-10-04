mod recent_counter;

use std::cmp::{max, min};

#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 624. Maximum Distance in Arrays.
    // https://leetcode.com/problems/maximum-distance-in-arrays/
    // Single-scan solution.
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        // Even though the first array has the min and the max, its range (max - min) is never assigned to res.
        let mut res = 0;
        let (mut min_a, mut max_b) = (arrays[0][0], arrays[0][arrays[0].len() - 1]);
        for i in 1..arrays.len() {
            let (a, b) = (arrays[i][0], arrays[i][arrays[i].len() - 1]);
            res = max(res, max((min_a - b).abs(), (a - max_b).abs()));
            min_a = min(min_a, a);
            max_b = max(max_b, b);
        }
        res
    }

    // 532. K-diff Pairs in an Array.
    // https://leetcode.com/problems/k-diff-pairs-in-an-array/
    // Sorting solution.
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut count = 0;
        let mut left = 0;
        let mut right = 1;
        while left < nums.len() && right < nums.len() {
            if left == right || nums[right] - nums[left] < k {
                right += 1;
            } else if nums[right] - nums[left] > k {
                left += 1;
            } else {
                count += 1;
                left += 1;
                while left < nums.len() && nums[left] == nums[left - 1] {
                    left += 1;
                }
            }
        }
        count
    }
}