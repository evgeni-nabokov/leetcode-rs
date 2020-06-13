use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;

// insert O(1)
// remove O(1)
// get_random O(n)

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