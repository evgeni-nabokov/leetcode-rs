use std::cmp::min;

pub struct MinStack {
    container: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack { container: Vec::with_capacity(64) }
    }

    pub fn push(&mut self, x: i32) {
        self.container.push((x, min(x, self.container.last().unwrap_or(&(x, x)).1)));
    }

    pub fn pop(&mut self) {
        self.container.pop();
    }

    pub fn top(&mut self) -> Option<i32> {
        if self.container.is_empty() {
            None
        } else {
            Some(self.container.last().unwrap().0)
        }
    }

    pub fn get_min(&self) -> Option<i32> {
        if self.container.is_empty() {
            None
        } else {
            Some(self.container.last().unwrap().1)
        }
    }
}

// https://github.com/wangyuntao/leetcode-rs
pub struct MinStack2 {
    v: Vec<i32>,
    m: Option<i32>,
}

impl MinStack2 {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        MinStack2 { v: vec![], m: None }
    }

    pub fn push(&mut self, x: i32) {
        self.v.push(x);
        if let Some(m) = self.m {
            if x < m {
                self.m = Some(x);
            }
        }
    }

    pub fn pop(&mut self) {
        let x = self.v.pop().unwrap();
        if let Some(m) = self.m {
            if x == m {
                self.m = None;
            }
        }
    }

    pub fn top(&self) -> i32 {
        return self.v[self.v.len() - 1];
    }

    pub fn get_min(&mut self) -> i32 {
        self.m.unwrap_or_else(|| {
            let m = self.v.iter().min().unwrap();
            let m = *m;
            self.m = Some(m);
            m
        })
    }
}


use std::collections::VecDeque;

pub struct MinStack3 {
    min: i32,
    stack: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack3 {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        MinStack3 {
            min: std::i32::MAX,
            stack: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        if x <= self.min {
            self.stack.push_front(self.min);
            self.min = x;
        }
        self.stack.push_front(x);
    }

    pub fn pop(&mut self) {
        if !self.stack.is_empty() {
            if self.min == self.stack.pop_front().unwrap() {
                self.min = self.stack.pop_front().unwrap();
            }
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.front().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        self.min
    }
}