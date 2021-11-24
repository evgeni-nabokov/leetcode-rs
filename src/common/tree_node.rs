use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn from_level_order(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let mut queue = VecDeque::new();
        let root = TreeNode::new_with_children(values[0], None, None);
        queue.push_back(root.clone());
        let mut i = 1;
        while !queue.is_empty() && i < values.len() {
            let mut curr = queue.pop_front().unwrap();
            let left = TreeNode::new_with_children(values[i], None, None);
            if left.is_some() {
                queue.push_back(left.clone());
            }
            curr.as_mut().unwrap().borrow_mut().left = left;
            i += 1;

            if i == values.len() {
                break;
            }

            let right = TreeNode::new_with_children(values[i], None, None);
            if right.is_some() {
                queue.push_back(right.clone());
            }
            curr.as_mut().unwrap().borrow_mut().right = right;
            i += 1;
        }

        root
    }

    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn new_with_children(val: Option<i32>, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>)
                  -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(inner_val) = val {
            Some(Rc::new(RefCell::new(TreeNode {
                val: inner_val,
                left,
                right,
            })))
        } else {
            None
        }
    }

}

pub trait BinaryTree {
    fn get_val(&self) -> i32;

    fn set_val(&mut self, val: i32);

    fn set_left_val(&self, val: i32) -> Option<Rc<RefCell<TreeNode>>>;

    fn set_right_val(&self, val: i32) -> Option<Rc<RefCell<TreeNode>>>;

    fn get_left_node(&self) -> Option<Rc<RefCell<TreeNode>>>;

    fn get_right_node(&self) -> Option<Rc<RefCell<TreeNode>>>;

    fn remove_left_node(&self);

    fn remove_right_node(&self);

    fn get_height(&self) -> usize;

    fn get_level_order_values(&self) -> Vec<Option<i32>>;
}

impl BinaryTree for Option<Rc<RefCell<TreeNode>>> {
    fn get_val(&self) -> i32 {
        match self {
            Some(some) => some.borrow().val,
            None => panic!("Node is None"),
        }
    }

    fn set_val(&mut self, val: i32) {
        match self {
            Some(some) => {
                RefCell::borrow_mut(some).val  = val;
            },
            None => panic!("Node is None"),
        };
    }

    fn set_left_val(&self, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match self {
            Some(some) => {
                let new_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                RefCell::borrow_mut(some).left  = new_node.clone();
                new_node
            },
            None => panic!("Node is None"),
        }
    }

    fn set_right_val(&self, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match self {
            Some(some) => {
                let new_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                RefCell::borrow_mut(some).right  = new_node.clone();
                new_node
            },
            None => panic!("Node is None"),
        }
    }

    fn get_left_node(&self) -> Option<Rc<RefCell<TreeNode>>> {
        match self {
            Some(some) => RefCell::borrow(some).left.clone(),
            None => panic!("Node is None"),
        }
    }

    fn get_right_node(&self) -> Option<Rc<RefCell<TreeNode>>> {
        match self {
            Some(some) => RefCell::borrow(some).right.clone(),
            None => panic!("Node is None"),
        }
    }

    fn remove_left_node(&self) {
        match self {
            Some(some) => RefCell::borrow_mut(some).right  = None,
            None => panic!("Node is None"),
        };
    }

    fn remove_right_node(&self) {
        match self {
            Some(some) => RefCell::borrow_mut(some).right  = None,
            None => panic!("Node is None"),
        };
    }

    fn get_height(&self) -> usize {
        fn get_height_rec(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
            if node.is_some() {
                let left_h = get_height_rec(&node.get_left_node());
                let right_h = get_height_rec(&node.get_right_node());
                max(left_h, right_h) + 1
            } else {
                0
            }
        }
        get_height_rec(self)
    }

    fn get_level_order_values(&self) -> Vec<Option<i32>> {
        let mut res = Vec::new();
        if self.is_none() { return res; }
        let mut queue = VecDeque::new();
        queue.push_back(self.clone());
        let mut tail_len = 0;
        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                if let Some(inner_node) = queue.pop_front().unwrap() {
                    let borrowed_node = inner_node.borrow();
                    res.push(Some(borrowed_node.val));
                    queue.push_back(borrowed_node.left.clone());
                    queue.push_back(borrowed_node.right.clone());
                    tail_len = 0;
                } else {
                    res.push(None);
                    tail_len += 1;
                }
            }
        }

        if tail_len > 0 {
            res.truncate(res.len() - tail_len);
        }

        res
    }

}