#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 2089. Find Target Indices After Sorting Array.
    // https://leetcode.com/problems/find-target-indices-after-sorting-array/
    // Time complexity: O(N*LogN).
    // Space complexity: O(1).
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_unstable();

        let mut left = 0;
        let mut right = nums.len();
        loop {
            match nums[left..right].binary_search(&target) {
                Ok(i) => right = i,
                Err(i) => {
                    left = i;
                    break;
                },
            }
        }

        let mut res = Vec::new();
        while left < nums.len() && nums[left] == target {
            res.push(left as i32);
            left += 1;
        }

        res
    }
}