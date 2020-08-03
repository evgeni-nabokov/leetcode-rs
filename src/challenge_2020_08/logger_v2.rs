// 359. Logger Rate Limiter.
// https://leetcode.com/problems/logger-rate-limiter/

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

pub struct Logger {
    map: HashMap<String, i32>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            map: HashMap::default(),
        }
    }

    pub fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        match self.map.entry(message) {
            Vacant(v) => {
                v.insert(timestamp);
                true
            },
            Occupied(mut entry) if timestamp - *entry.get() >= 10 => {
                *entry.get_mut() = timestamp;
                true
            }
            _ => false
        }
    }
}