// 380. Insert Delete GetRandom O(1).
// https://leetcode.com/problems/insert-delete-getrandom-o1/

// insert O(1)
// remove O(1)
// get_random O(n) <-- not good.

use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {

    pub fn new() -> Self {
        RandomizedSet {
            set: HashSet::new()
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        self.set.insert(val)
    }

    pub fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }

    pub fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let a: Vec<_> = self.set.iter().collect();
        **a.choose(&mut rng).unwrap()
    }
}