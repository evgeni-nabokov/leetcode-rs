// 173. Binary Search Tree Iterator.
// https://leetcode.com/problems/binary-search-tree-iterator/
// Time complexity: O(N).
// Space complexity: O(N).

use std::rc::Rc;
use std::cell::RefCell;

use crate::common::tree_node::TreeNode;

pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut res = BSTIterator {
            stack: Vec::new(),
        };

        res.push_leftmost(root);
        res
    }

    pub fn next(&mut self) -> i32 {
        let mut node = self.stack.pop().unwrap();
        let res = node.borrow().val;
        self.push_leftmost(node.borrow_mut().right.take());
        res
    }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn push_leftmost(&mut self, mut node: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(inner_node) = node {
            node = inner_node.borrow_mut().left.take();
            self.stack.push(inner_node);
        }
    }
}