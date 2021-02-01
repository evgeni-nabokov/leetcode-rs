use std::cmp::min;

pub struct MinStack {
    container: Vec<(i32, i32)>
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            container: Vec::with_capacity(64),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.container.push((x, min(x, self.container.last().unwrap_or(&(x, x)).1)));
    }

    pub fn pop(&mut self) {
        self.container.pop();
    }

    pub fn top(&self) -> i32 {
        self.container.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
        self.container.last().unwrap().1
    }
}