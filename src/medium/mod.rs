#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

use crate::common::tree_node::TreeNode;

struct Solution {}

impl Solution {
    // 274. H-Index
    // https://leetcode.com/problems/h-index/
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        if citations.is_empty() { return 0; }
        citations.sort_unstable();
        let l = citations.len() as i32;
        let mut left = 0;
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

    // 1325. Delete Leaves With a Given Value.
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

    // 102. Binary Tree Level Order Traversal.
    // https://leetcode.com/problems/binary-tree-level-order-traversal/
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
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
        levels
    }

    // 889. Construct Binary Tree from Preorder and Postorder Traversal.
    // https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
    pub fn build_tree(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_bt(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || postorder.is_empty() { return None; }
            let mut val = preorder[0];
            let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            if preorder.len() == 1 { return node; }
            val = preorder[1];
            let i = postorder.iter().position(|x| *x == val).unwrap();
            RefCell::borrow_mut(node.as_ref().unwrap()).left = build_bt(&preorder[1..=(1 + i)], &postorder[..=i]);
            RefCell::borrow_mut(node.as_ref().unwrap()).right = build_bt(&preorder[(i + 2)..], &postorder[(i + 1)..(postorder.len() - 1)]);
            node
        }

        build_bt(&preorder, &postorder)
    }
}
