#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

use crate::common::tree_node::TreeNode;

struct Solution {}

impl Solution {
    // 15. 3Sum
    // https://leetcode.com/problems/3sum/
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 { return Vec::new(); }
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort_unstable();
        let n: usize = nums.len();
        for a_index in 0..=n - 2 {
            if a_index > 0 && nums[a_index] == nums[a_index - 1] { continue; }
            let a = nums[a_index];
            let mut b_index = a_index + 1 as usize;
            let mut c_index = n - 1 as usize;
            while b_index < c_index {
                let b = nums[b_index];
                let c = nums[c_index];
                match a + b + c {
                    0 => {
                        result.push(vec![a, b, c]);
                        while { b_index += 1; b_index < c_index && nums[b_index] == nums[b_index - 1] }{};
                        while { c_index -= 1; b_index < c_index && nums[c_index] == nums[c_index + 1] }{};
                    },
                    x if x > 0 => {
                        while { c_index -= 1; b_index < c_index && nums[c_index] == nums[c_index + 1] }{};
                    }
                    x if x < 0 => {
                        while { b_index += 1; b_index < c_index && nums[b_index] == nums[b_index - 1] }{};
                    },
                    _ => ()
                }
            }
        }
        result
    }

    // 274. H-Index
    // https://leetcode.com/problems/h-index/
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        if citations.is_empty() { return 0; }
        citations.sort_unstable();
        let l = citations.len() as i32;
        let mut left= 0;
        let mut right = l - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let h = l - mid;
            match citations[mid as usize].cmp(&h) {
                Ordering::Equal => return h,
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
            }
        }
        l - left
    }

    // 1325. Delete Leaves With a Given Value
    // https://leetcode.com/problems/delete-leaves-with-a-given-value/
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(some) = root {
            let left = Solution::remove_leaf_nodes(RefCell::borrow_mut(&some).left.clone(), target);
            let right = Solution::remove_leaf_nodes(RefCell::borrow_mut(&some).right.clone(), target);
            if left.is_none() && right.is_none() && RefCell::borrow(&some).val == target {
                None
            } else {
                RefCell::borrow_mut(&some).left = left.clone();
                RefCell::borrow_mut(&some).right = right.clone();
                Some(some)
            }
        } else {
            None
        }
    }

    // 79. Word Search.
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(board: &mut Vec<Vec<char>>, word: &[char], i: usize, r: isize, c: isize) -> bool {
            if i >= word.len() { return false; }
            if r < 0 || c < 0 || r as usize >= board.len() || c as usize >= board[0].len() { return false; }

            let ur = r as usize;
            let uc = c as usize;
            let ch = board[ur][uc];
            if word[i] != ch { return false; }
            if i == word.len() - 1 { return true; }

            board[ur][uc] = '#';
            for (x, y) in vec![(0, -1), (0, 1), (1, 0), (-1, 0)].into_iter() {
                if dfs(board, word,  i + 1, r + y, c + x) {
                    return true;
                }
            }
            board[ur][uc] = ch;
            false
        }

        for r in 0..board.len() as isize{
            for c in 0..board[0].len() as isize {
                if dfs(&mut board, &word.chars().collect::<Vec<char>>(), 0, r, c) {
                    return true;
                }
            }
        }
        false
    }
}