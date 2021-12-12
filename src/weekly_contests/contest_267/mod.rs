#[cfg(test)]
mod tests;

struct Solution;

impl Solution {

    // 2073. Time Needed to Buy Tickets.
    // https://leetcode.com/problems/time-needed-to-buy-tickets/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let k_idx = k as usize;
        loop {
            for i in 0..tickets.len() {
                if tickets[i] == 0 {
                    continue;
                }
                tickets[i] -= 1;
                res += 1;
                if i == k_idx && tickets[i] == 0 {
                    return res;
                }
            }
        }
    }
}