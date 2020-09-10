// 346. Moving Average from Data Stream.
// https://leetcode.com/problems/moving-average-from-data-stream/

use std::collections::VecDeque;

pub struct MovingAverage {
    size: usize,
    sum: i32,
    list: VecDeque<i32>,
}

impl MovingAverage {
    pub fn new(size: i32) -> Self {
        MovingAverage {
            size: size as usize,
            list: VecDeque::with_capacity(size as usize),
            sum: 0,
        }
    }

    pub fn next(&mut self, val: i32) -> f64 {
        self.list.push_back(val);
        self.sum += val;
        if self.list.len() > self.size {
            self.sum -= self.list.pop_front().unwrap();
        }
        self.sum as f64 / self.list.len() as f64
    }
}