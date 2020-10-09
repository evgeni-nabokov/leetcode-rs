// 170. Two Sum III - Data structure design.
// https://leetcode.com/problems/two-sum-iii-data-structure-design/

use std::collections::HashMap;

// HashMap solution.
pub struct TwoSum {
    map: HashMap<i32, usize>
}

impl TwoSum {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    /** Add the number to an internal data structure.. */
    pub fn add(&mut self, number: i32) {
        *self.map.entry(number).or_insert(0) += 1
    }

    /** Find if there exists any pair of numbers which sum is equal to the value. */
    pub fn find(&self, value: i32) -> bool {
        for n in self.map.keys() {
            let m = value - *n;
            if let Some(count) = self.map.get(&m) {
                if m != *n || *count > 1 {
                    return true;
                }
            }
        }
        false
    }
}