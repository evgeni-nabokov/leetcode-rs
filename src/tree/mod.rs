#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;

struct Solution;

impl Solution {
    // 112. Path Sum.
    // https://leetcode.com/problems/path-sum/
    // Recursive DFS method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> bool {
            if let Some(inner_node) = node {
                let borr_node = inner_node.borrow();
                sum -= borr_node.val;
                if borr_node.left.is_none() && borr_node.right.is_none() {
                    sum == 0
                } else {
                    dfs(&borr_node.left, sum) || dfs(&borr_node.right, sum)
                }
            } else {
                false
            }
        }

        dfs(&root, target_sum)
    }

    // Iterative DFS method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn has_path_sum_v2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let root = root.unwrap();
        let val = root.borrow().val;
        let mut stack = vec![(root, target_sum - val)];
        while !stack.is_empty() {
            let (node, sum) = stack.pop().unwrap();
            if node.borrow().left.is_none() && node.borrow().right.is_none() && sum == 0 {
                return true;
            }
            if node.borrow().left.is_some() {
                let left = node.borrow_mut().left.take().unwrap();
                let val = left.borrow().val;
                stack.push((left, sum - val));
            }
            if node.borrow().right.is_some() {
                let right = node.borrow_mut().right.take().unwrap();
                let val = right.borrow().val;
                stack.push((right, sum - val));
            }
        }

        false
    }

    // 113. Path Sum II.
    // https://leetcode.com/problems/path-sum-ii/
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            mut sum: i32,
            path: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if let Some(inner_node) = node {
                let borr_node = inner_node.borrow();
                path.push(borr_node.val);
                sum -= borr_node.val;
                if borr_node.left.is_none() && borr_node.right.is_none() && sum == 0 {
                    res.push(path.clone());
                } else {
                    dfs(&borr_node.left, sum, path, res);
                    dfs(&borr_node.right, sum, path, res);
                }
                path.pop();
            }
        }

        let mut res = Vec::new();
        dfs(&root, target_sum, &mut Vec::new(), &mut res);
        res
    }
}
