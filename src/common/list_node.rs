#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    #[inline]
    pub fn new_with_next(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            next,
            val,
        }
    }

    pub fn from_slice(values: &[i32]) -> Option<Box<ListNode>> {
        if values.is_empty() { return None; }
        let mut node: Option<Box<ListNode>>;
        let mut next_node = Some(Box::new(ListNode::new(*values.last().unwrap())));
        for v in values.iter().rev().skip(1) {
            node = Some(Box::new(ListNode::new_with_next(*v, next_node)));
            next_node = node;
        }
        next_node
    }
}