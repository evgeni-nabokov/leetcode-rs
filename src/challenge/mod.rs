use std::collections::HashSet;

#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/single-number/
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.push(0); // A stub for one more iteration.
        let mut prev: &i32 = &nums[0];
        let mut cnt: u32 = 1;
        for n in nums.iter().skip(1) {
            if *n == *prev {
                cnt += 1;
            } else {
                if cnt == 1 {
                    return *prev;
                } else {
                    cnt = 1;
                    prev = n;
                }
            }
        }
        0
    }

    pub fn single_number_v2(nums: Vec<i32>) -> i32 {
        let mut a: i32 = 0;
        for n in nums.iter() {
            a ^= *n;
        }
        a
    }

    // https://leetcode.com/problems/happy-number/
    pub fn is_happy(n: i32) -> bool {
        if n == 0 { return false; }
        if n == 1 { return true; }
        let mut nn = n.clone();
        let mut sum_set: HashSet<i32> = HashSet::with_capacity(20);
        sum_set.insert(nn);
        let mut sum = 0;
        loop {
            while nn > 0 {
                let d = nn % 10;
                sum += d * d;
                nn = nn / 10;
            }
            if sum == 1 {
                return true;
            } else if sum_set.contains(&sum) {
                return false;
            }
            sum_set.insert(sum);
            nn = sum;
            sum = 0;
        }
    }

    // https://leetcode.com/problems/maximum-subarray/
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        let mut sum: i32 = nums[0];
        let mut max_sum: i32 = sum;
        for &n in nums.iter().skip(1) {
            match n {
                n if n >= 0 && sum < 0 => sum = n,
                n if n >= 0 && sum >= 0 => sum += n,
                n if n < 0 && sum < 0 && sum < n => sum = n,
                n if n < 0 && sum > 0 => {
                    if sum > max_sum {
                        max_sum = sum;
                    }
                    sum += n;
                }
                _ => ()
            }
        }
        if sum > max_sum {
            max_sum = sum;
        }
        max_sum
    }
}