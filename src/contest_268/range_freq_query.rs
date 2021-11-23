// 2080. Range Frequency Queries
// https://leetcode.com/problems/range-frequency-queries/
// Time complexity: O(N) for initialization and O(LogN) for query.
// Space complexity: O(N).

use std::collections::HashMap;

pub struct RangeFreqQuery {
    freq: HashMap<i32, Vec<i32>>,
}

impl RangeFreqQuery {
    pub fn new(arr: Vec<i32>) -> Self {
        let mut freq = HashMap::new();

        for i in 0..arr.len() {
            freq.entry(arr[i]).or_insert(vec![]).push(i as i32);
        }

        RangeFreqQuery {
            freq,
        }
    }

    pub fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        match self.freq.get(&value) {
            Some(indices) => {
                let l = match indices.binary_search(&left) {
                    Ok(i) => i,
                    Err(i) => i,
                } as i32;
                let r = match indices.binary_search(&right) {
                    // Return the point that after (to the right of) any existing entries.
                    Ok(i) => i + 1,
                    Err(i) => i,
                } as i32;
                r - l
            },
            _ => 0,
        }
    }
}
