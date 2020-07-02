#[cfg(test)]
mod tests;

struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        (((8f64 * n as f64 + 1f64).sqrt() - 1f64) / 2f64).floor() as i32
    }
}