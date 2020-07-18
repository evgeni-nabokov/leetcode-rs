// 528. Random Pick with Weight.
// https://leetcode.com/problems/random-pick-with-weight/

use rand::{Rng, thread_rng};

pub struct Solution {
    points: Vec<u64>,
}

impl Solution {
    pub fn new(weights: Vec<i32>) -> Self {
        let mut points: Vec<u64>  = Vec::with_capacity(weights.len() + 1);
        points.push(0);
        let mut sum = 0u64;
        for w in weights {
            sum += w as u64;
            points.push(sum);
        }
        Solution {
            points,
        }
    }

    pub fn pick_index(&self) -> i32 {
        let mut rng = thread_rng();
        let n = rng.gen_range(0, self.points.last().unwrap());
        let i = match self.points.binary_search(&n) {
            Ok(i) => i as i32,
            Err(i) => i as i32 - 1,
        };
        i as i32
    }
}
