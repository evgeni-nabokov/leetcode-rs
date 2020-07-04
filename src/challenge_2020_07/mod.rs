#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::common::tree_node::{TreeNode};

struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        (((8f64 * n as f64 + 1f64).sqrt() - 1f64) / 2f64).floor() as i32
    }

    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<Vec<i32>>, level: usize) {
            if node.is_none() { return; }
            if level == levels.len() {
                levels.push(vec![]);
            }
            levels[level].push(RefCell::borrow(node.as_ref().unwrap()).val);
            dfs(RefCell::borrow(node.as_ref().unwrap()).left.clone(), levels, level + 1);
            dfs(RefCell::borrow(node.as_ref().unwrap()).right.clone(), levels, level + 1);
        }

        let mut levels: Vec<Vec<i32>> = Vec::new();
        dfs(root, &mut levels, 0);
        levels.into_iter().rev().collect()
    }

    pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
        if cells.is_empty() { return vec![]; }
        if n == 0 { return cells; }
        let l = cells.len();
        let mut state_vec: Vec<Vec<i32>> = Vec::new();
        let mut state_map: HashMap<Vec<i32>, usize> = HashMap::new();
        let mut i = 0usize;
        let mut prev_c = cells[0];
        loop {
            for c in 1..l - 1 {
                if prev_c + cells[c + 1] == 1 {
                    prev_c = cells[c];
                    cells[c] = 0;
                } else {
                    prev_c = cells[c];
                    cells[c] = 1;
                }
            }
            cells[0] = 0;
            cells[l - 1] = 0;
            prev_c = 0;
            i += 1;
            if i == n as usize {
                return cells;
            }
            if let Some(start) = state_map.get(&cells) {
                let j = (n as usize - *start) % (i - *start);
                if j == 0 {
                    return cells
                }
                return state_vec[j].clone()
            }
            state_vec.push(cells.clone());
            state_map.insert(cells.clone(), i);
        }
    }

    pub fn prison_after_n_days_v2(mut cells: Vec<i32>, mut n: i32) -> Vec<i32> {
        if cells.is_empty() { return vec![]; }
        if n == 0 { return cells; }
        let l = cells.len();
        let mut state_map: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut prev_c = cells[0];
        let mut is_fast_forwarded = false;
        while n > 0 {
            if !is_fast_forwarded {
                if let Some(start) = state_map.get(&cells) {
                    n %= (*start as i32 - n);
                    is_fast_forwarded = true;
                } else {
                    state_map.insert(cells.clone(), n);
                }
            }
            if n > 0 {
                for c in 1..l - 1 {
                    if prev_c + cells[c + 1] == 1 {
                        prev_c = cells[c];
                        cells[c] = 0;
                    } else {
                        prev_c = cells[c];
                        cells[c] = 1;
                    }
                }
                cells[0] = 0;
                cells[l - 1] = 0;
                prev_c = 0;
                n -= 1;
            }
        }
        cells
    }
}