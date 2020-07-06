#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp::min;

use lazy_static::lazy_static;

use crate::common::tree_node::TreeNode;

lazy_static! {
    static ref UGLY_NUMBERS: Vec<i32> = {
        let mut n = 1690;
        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;
        let mut nums: Vec<i32> = Vec::with_capacity(n);
        nums.push(1);
        n -= 1;
        while n > 0 {
            let u = min(min(nums[i2] * 2, nums[i3] * 3), nums[i5] * 5);
            if u == nums[i2] * 2 {
                i2 += 1;
            }
            if u == nums[i3] * 3 {
                i3 += 1;
            }
            if u == nums[i5] * 5 {
                i5 += 1;
            }
            nums.push(u);
            n -= 1;
        }
        nums
    };
}

struct Solution {}

impl Solution {
    // 441. Arranging Coins.
    // https://leetcode.com/problems/arranging-coins/
    pub fn arrange_coins(n: i32) -> i32 {
        (((8f64 * n as f64 + 1f64).sqrt() - 1f64) / 2f64).floor() as i32
    }

    // 107. Binary Tree Level Order Traversal II.
    // https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
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

    // 957. Prison Cells After N Days.
    // https://leetcode.com/problems/prison-cells-after-n-days/
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
                    n %= *start as i32 - n;
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

    // 264. Ugly Number II.
    // https://leetcode.com/problems/ugly-number-ii/
    pub fn nth_ugly_number(n: i32) -> i32 {
        UGLY_NUMBERS[n as usize - 1]
    }

    // 461. Hamming Distance.
    // https://leetcode.com/problems/hamming-distance/
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut n = 1;
        let mut cnt = 0;
        for _ in 1..32 {
            if x & n != y & n {
                cnt += 1;
            }
            n <<= 1;
        }
        cnt
    }

    pub fn hamming_distance_v2(x: i32, y: i32) -> i32 {
        let mut z = x ^ y;
        let mut distance = 0;
        while z != 0 {
            distance += 1;
            z = z & (z - 1);
        }
        distance
    }

    pub fn hamming_distance_v3(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }

    // 66. Plus One.
    // https://leetcode.com/problems/plus-one/
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut i: isize = digits.len() as isize - 1;
        while i >= 0 {
            let ui = i as usize;
            if digits[ui] == 9 {
                digits[ui] = 0;
                i -= 1
            } else {
                digits[ui] += 1;
                break;
            }
        }
        if i < 0 {
            digits.push(0);
            digits[0] = 1;
        }
        digits
    }
}
