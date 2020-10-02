// 933. Number of Recent Calls.
// https://leetcode.com/problems/number-of-recent-calls/

use std::collections::VecDeque;

pub struct RecentCounter {
    queue: VecDeque<i32>
}

impl RecentCounter {
    const INIT_CAPACITY: usize = 100;
    const TIME_FRAME: i32 = 3000;

    pub fn new() -> Self {
        RecentCounter {
            queue: VecDeque::with_capacity(RecentCounter::INIT_CAPACITY)
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        while !self.queue.is_empty() && *self.queue.back().unwrap() < t - RecentCounter::TIME_FRAME {
            self.queue.pop_back();
        }
        self.queue.push_front(t);
        self.queue.len() as i32
    }
}