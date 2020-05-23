use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn create_from_level_order(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        fn insert_level_order(values: &[Option<i32>], i: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if i >= values.len() {
                None
            } else {
                match values[i] {
                    Some(val) =>
                        TreeNode::new_with_children(
                            val,
                            insert_level_order(values, 2 * i + 1),
                            insert_level_order(values, 2 * i + 2)
                        ),
                    None => None
                }
            }
        }
        insert_level_order(values, 0)
    }

    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }

    pub fn new_with_children(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>)
                  -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left,
            right,
        })))
    }

}

pub trait BinaryTree {
    fn get_val(&self) -> Option<i32>;

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
    fn get_val(&self) -> Option<i32> {
        self.as_ref().map(|some| some.borrow().val)
    }

    fn set_val(&mut self, val: i32) {
        match self {
            Some(some) => {
                RefCell::borrow_mut(some).val  = val;
            },
            None => panic!("Node is None"),
            // None => {
            //     replace(self, Some(Rc::new(RefCell::new(TreeNode::new(val)))));
            // }
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
                if left_h > right_h {
                    left_h + 1
                } else {
                    right_h + 1
                }
            } else {
                0
            }
        }
        get_height_rec(self)
    }

    fn get_level_order_values(&self) -> Vec<Option<i32>> {
        fn get_given_level(node: &Option<Rc<RefCell<TreeNode>>>, level: usize) -> Vec<Option<i32>> {
            let mut res = Vec::<Option<i32>>::new();
            match level {
                1 => res.push(node.get_val()),
                _ => {
                    res.append(get_given_level(&node.get_left_node(), level - 1).as_mut());
                    res.append(get_given_level(&node.get_right_node(), level - 1).as_mut());
                }
            };
            res
        }
        let height = self.get_height();
        let mut res = Vec::<Option<i32>>::with_capacity(2 ^ (height - 1));
        for i in 1..=height {
            res.append(get_given_level(&self, i).as_mut())
        }
        res
    }
}