use std::collections::HashMap;

#[cfg(test)]
mod tests;

struct Solution {}

// https://leetcode.com/problems/two-sum/
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache: HashMap<i32, usize> = HashMap::new();
        cache.insert(nums[0], 0);
        for i in 1..nums.len() {
            let a = target - nums[i];
            if cache.contains_key(&a) {
                return vec![cache[&a] as i32, i as i32]
            } else {
                cache.insert(nums[i], i);
            }
        }
        vec![]
    }

    // https://leetcode.com/problems/reverse-integer/
    pub fn reverse(x: i32) -> i32 {
        let mut xx = x.clone();
        let mut rx = 0i32;
        while xx != 0 {
            if let Some(t) = rx.checked_mul(10) {
                rx = t;
            } else {
                return 0;
            }
            if let Some(t) = rx.checked_add(xx % 10) {
                rx = t;
            } else {
                return 0;
            }
            xx = xx / 10;
        }
        rx
    }
}