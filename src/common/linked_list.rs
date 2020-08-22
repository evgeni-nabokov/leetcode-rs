use crate::common::list_node::ListNode;

pub trait LinkedList {
    fn to_vec(&self) -> Vec<i32>;
}

impl LinkedList for Option<Box<ListNode>> {
    fn to_vec(&self) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut node = self.clone();
        while let Some(some) = node {
            res.push(some.val);
            node = some.next;
        }
        res
    }
}