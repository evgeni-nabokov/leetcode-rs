use std::collections::HashSet;
use std::cmp::max;

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

    pub fn max_sub_array_v2(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        let mut sum: i32 = nums[0];
        let mut max_sum: i32 = sum;
        for &n in nums.iter().skip(1) {
            sum = max(n, sum + n);
            max_sum = max(sum, max_sum);
        }
        max_sum
    }

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() < 2 { return; }
        let mut i: usize = 0;
        let mut cnt: usize = 0;
        let len = nums.len();
        loop {
            if i == len {
                break;
            } else if nums[i] == 0 {
                cnt += 1;
            } else if cnt > 0 {
                nums[i - cnt] = nums[i];
                nums[i] = 0;
            }
            i += 1;
        }
    }

    pub fn move_zeroes_v2(nums: &mut Vec<i32>) {
        if nums.len() < 2 { return; }
        let mut main_index: usize = 0;
        let mut non_zero_index: usize = 0;
        let len = nums.len();
        loop {
            if main_index == len {
                break;
            } else if nums[main_index] != 0 {
                nums.swap(non_zero_index, main_index);
                non_zero_index += 1;
            }
            main_index += 1;
        }
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 { return 0; }
        let mut in_tran = false;
        let mut i: usize = 0;
        let last_i = prices.len() - 1;
        let mut buy_price = 0;
        let mut profit = 0;
        loop {
            if i == last_i {
                if in_tran {
                    // Final selling.
                    profit += prices[i] - buy_price;
                }
                break;
            }
            if in_tran {
                if prices[i] < prices[i + 1] {
                    // Do nothing.
                } else {
                    // Selling.
                    profit += prices[i] - buy_price;
                    in_tran = false;
                }
            } else {
                if prices[i] < prices[i + 1] {
                    // Buying.
                    buy_price = prices[i];
                    in_tran = true;
                } else {
                    // Do nothing.
                }
            }
            i += 1;
        }
        profit
    }

    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 { return 0; }
        let mut i: usize = 1;
        let len = prices.len();
        let mut profit = 0;
        while i < len {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
            i += 1;
        }
        profit
    }
}