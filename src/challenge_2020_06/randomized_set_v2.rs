// 380. Insert Delete GetRandom O(1).
// https://leetcode.com/problems/insert-delete-getrandom-o1/

// insert O(1)
// remove O(1)
// get_random O(1)

use std::collections::HashMap;
use std::collections::hash_map::Entry::Vacant;
use rand::{Rng, thread_rng};

pub struct RandomizedSet {
    map: HashMap<i32, usize>,
    v: Vec<i32>,
}

impl RandomizedSet {

    pub fn new() -> Self {
        RandomizedSet {
            map: HashMap::new(),
            v: Vec::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        return match self.map.entry(val) {
            Vacant(v) => {
                v.insert(self.v.len());
                self.v.push(val);
                true
            },
            _ => false,
        }
    }

    pub fn remove(&mut self, val: i32) -> bool {
        return match self.map.remove(&val) {
            Some(i) => {
                if i == self.v.len() - 1 {
                    self.v.pop();
                } else {
                    self.v.swap_remove(i);
                    *self.map.get_mut(&self.v[i]).unwrap() = i;
                }
                true
            },
            _ => false,
        }
    }

    pub fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        self.v[rng.gen_range(0, self.v.len())]
    }

}