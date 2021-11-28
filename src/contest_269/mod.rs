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

    // 2090. K Radius Subarray Averages.
    // https://leetcode.com/problems/k-radius-subarray-averages/
    // Prefix sum method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let uk = k as usize;
        let mut res = vec![-1; nums.len()];

        if uk == 0 {
            return nums;
        }

        if 2 * uk >= nums.len()  {
            return res;
        }

        let mut prefix_sum = vec![0u64; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as u64;
        }

        let n = 2 * k as u64 + 1;
        for i in uk..nums.len() - uk {
            let left = i - uk;
            let right = i + uk + 1;
            res[i] = ((prefix_sum[right] - prefix_sum[left]) / n) as i32;
        }

        res
    }

    // Window sliding method.
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn get_averages_v2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let uk = k as usize;
        let mut res = vec![-1; nums.len()];

        if uk == 0 {
            return nums;
        }

        if 2 * uk >= nums.len()  {
            return res;
        }

        let n = 2 * k as i64 + 1;
        let mut sum = 0;
        for i in 0..(2 * uk + 1) {
            sum += nums[i] as i64;
        }

        res[uk] = (sum / n) as i32;

        for i in (uk + 1)..nums.len() - uk {
            sum += (nums[i + uk] - nums[i - uk - 1]) as i64;
            res[i] = (sum / n) as i32;
        }

        res
    }
}