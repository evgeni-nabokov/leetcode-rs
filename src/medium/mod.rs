#[cfg(test)]
mod tests;

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    // 15. 3Sum
    // https://leetcode.com/problems/3sum/
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 { return Vec::new(); }
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort_unstable();
        let n: usize = nums.len();
        for a_index in 0..=n - 2 {
            if a_index > 0 && nums[a_index] == nums[a_index - 1] { continue; }
            let a = nums[a_index];
            let mut b_index = a_index + 1 as usize;
            let mut c_index = n - 1 as usize;
            while b_index < c_index {
                let b = nums[b_index];
                let c = nums[c_index];
                match a + b + c {
                    0 => {
                        result.push(vec![a, b, c]);
                        while { b_index += 1; b_index < c_index && nums[b_index] == nums[b_index - 1] }{};
                        while { c_index -= 1; b_index < c_index && nums[c_index] == nums[c_index + 1] }{};
                    },
                    x if x > 0 => {
                        while { c_index -= 1; b_index < c_index && nums[c_index] == nums[c_index + 1] }{};
                    }
                    x if x < 0 => {
                        while { b_index += 1; b_index < c_index && nums[b_index] == nums[b_index - 1] }{};
                    },
                    _ => ()
                }
            }
        }
        result
    }

    // 274. H-Index
    // https://leetcode.com/problems/h-index/
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        if citations.is_empty() { return 0; }
        citations.sort_unstable();
        let l = citations.len() as i32;
        let mut left= 0;
        let mut right = l - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let h = l - mid;
            match citations[mid as usize].cmp(&h) {
                Ordering::Equal => return h,
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
            }
        }
        l - left
    }
}