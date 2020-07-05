use std::collections::HashMap;

#[cfg(test)]
mod tests;

struct Solution {}

impl Solution {
    // 1. Two Sum.
    // https://leetcode.com/problems/two-sum/
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

    // 7. Reverse Integer.
    // https://leetcode.com/problems/reverse-integer/
    pub fn reverse(x: i32) -> i32 {
        let mut xx = x;
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

    // 1108. Defanging an IP Address.
    // https://leetcode.com/problems/defanging-an-ip-address/
    pub fn defang_ip_addr(address: String) -> String {
        address.replace(".", "[.]")
    }

    // 1342. Number of Steps to Reduce a Number to Zero (Easy).
    // https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
    pub fn number_of_steps (num: i32) -> i32 {
        let mut cnt = 0;
        let mut n = num;
        while n != 0 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n -= 1;
            }
            cnt += 1;
        }
        cnt
    }
}