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

    // 2091. Removing Minimum and Maximum From Array.
    // https://leetcode.com/problems/removing-minimum-and-maximum-from-array/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut min_n = nums[0];
        let mut max_n = nums[0];
        let mut min_i = 0;
        let mut max_i = 0;

        for i in 1..nums.len() {
            if nums[i] > max_n {
                max_n = nums[i];
                max_i = i;
            } else if nums[i] < min_n {
                min_n = nums[i];
                min_i = i
            }
        }

        // Three cases:
        // 1) both numbers are in the left half - deletions are only from the left side,
        // 2) both numbers are in the right half - deletions are only from the right side,
        // 3) the numbers are in different halves - deletions are from the both sides.

        // Case 1 solution.
        let max_left_distance = max_i.max(min_i) + 1;

        // Case 2 solution.
        let max_right_distance = nums.len() - max_i.min(min_i);

        // Case 3 solution.
        let min_left_distance = max_i.min(min_i) + 1;
        let min_right_distance = nums.len() - max_i.max(min_i);

        // Find the minimum of the three cases.
        (min_left_distance + min_right_distance).min(max_left_distance.min(max_right_distance)) as i32
    }

    // 2092. Find All People With Secret.
    // https://leetcode.com/problems/find-all-people-with-secret/
    // Time complexity: O(N^2).
    // Space complexity: O(N).
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        // Set of people knowing the secret.
        let mut know_secret = vec![false; n as usize];

        know_secret[0] = true;
        know_secret[first_person as usize] = true;

        // Sorting by time in ascending order.
        meetings.sort_unstable_by_key(|x| x[2]);

        let mut left = 0;
        while left < meetings.len() {
            // We have a group of meetings at the same time.
            // Checking all the pairs of the meeting group.
            // If someone knows the secret, share it to another and set the flag secret_shared.
            // Do sharing rounds until no secret shared during the round.
            let time = meetings[left][2];
            let mut secret_shared = true;
            let mut right = left;
            while secret_shared {
                secret_shared = false;
                right = left;
                while right < meetings.len() && meetings[right][2] == time {
                    let x = meetings[right][0] as usize;
                    let y = meetings[right][1] as usize;
                    if know_secret[x] && !know_secret[y] || !know_secret[x] && know_secret[y] {
                        // We found a person that knows the secret and can share it.
                        secret_shared = true;
                        know_secret[x] = true;
                        know_secret[y] = true;
                    }
                    right += 1;
                }
            }
            left = right;
        }

        know_secret.into_iter().enumerate().filter(|x| x.1).map(|x| x.0 as i32).collect()
    }
}