#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::cell::RefCell;

use crate::common::tree_node::TreeNode;

struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn swap_children(parent: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(some) = parent {
                let left = RefCell::borrow_mut(&some).left.clone();
                let right = RefCell::borrow_mut(&some).right.clone();
                if left.is_none() && right.is_some() {
                    RefCell::borrow_mut(&some).left = right;
                    RefCell::borrow_mut(&some).right = None;
                    swap_children(RefCell::borrow_mut(&some).left.clone());
                } else if left.is_some() && right.is_none() {
                    RefCell::borrow_mut(&some).right = left;
                    RefCell::borrow_mut(&some).left = None;
                    swap_children(RefCell::borrow_mut(&some).right.clone());
                } else if left.is_some() && right.is_some() {
                    let node = RefCell::borrow(&some);
                    node.left.as_ref().unwrap().swap(node.right.as_ref().unwrap());
                    swap_children(node.left.clone());
                    swap_children(node.right.clone());
                }
                Some(some)
            } else {
                None
            }
        }
        swap_children(root)
    }
}