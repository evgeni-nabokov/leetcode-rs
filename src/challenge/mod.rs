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
}