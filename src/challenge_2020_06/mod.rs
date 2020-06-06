mod pick_index;

#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::cell::RefCell;

use crate::common::tree_node::TreeNode;

struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(some) = root {
            let left = RefCell::borrow_mut(&some).left.clone();
            let right = RefCell::borrow_mut(&some).right.clone();
            if left.is_none() && right.is_some() {
                RefCell::borrow_mut(&some).left = right;
                RefCell::borrow_mut(&some).right = None;
                Solution::invert_tree(RefCell::borrow_mut(&some).left.clone());
            } else if left.is_some() && right.is_none() {
                RefCell::borrow_mut(&some).right = left;
                RefCell::borrow_mut(&some).left = None;
                Solution::invert_tree(RefCell::borrow_mut(&some).right.clone());
            } else if left.is_some() && right.is_some() {
                let node = RefCell::borrow(&some);
                node.left.as_ref().unwrap().swap(node.right.as_ref().unwrap());
                Solution::invert_tree(node.left.clone());
                Solution::invert_tree(node.right.clone());
            }
            Some(some)
        } else {
            None
        }
    }

    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_unstable_by_key(|a| a[0] - a[1]);
        let n = costs.len() / 2;
        costs.iter().take(n).map(|x| x[0]).sum::<i32>() + costs.iter().skip(n).map(|x| x[1]).sum::<i32>()
    }

    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() { return; }
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}