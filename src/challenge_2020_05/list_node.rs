// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        res.push(self.val);
        let mut curr_node = self.next.clone();
        while curr_node.is_some() {
            let node = *curr_node.unwrap();
            res.push(node.val);
            curr_node = node.next;
        }
        res
    }

    pub fn from_slice(values: &[i32]) -> Option<Box<ListNode>> {
        if values.is_empty() { return None; }
        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(values[0])));
        if values.len() == 1 { return head; }
        let mut curr_node: &mut Option<Box<ListNode>> = &mut head;
        for &val in values.iter().skip(1) {
            curr_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
            curr_node = &mut curr_node.as_mut().unwrap().next;
        }
        head
    }
}