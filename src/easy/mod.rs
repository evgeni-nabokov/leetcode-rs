use std::collections::HashMap;
use crate::common::list_node::ListNode;

#[cfg(test)]
mod tests;

struct Solution {}

impl Solution {
    // 1. Two Sum.
    // https://leetcode.com/problems/two-sum/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        map.insert(nums[0], 0);
        for i in 1..nums.len() {
            let a = target - nums[i];
            if let Some(b) = map.get(&a) {
                return vec![*b as i32, i as i32]
            } else {
                map.insert(nums[i], i);
            }
        }
        vec![]
    }

    // 7. Reverse Integer.
    // https://leetcode.com/problems/reverse-integer/
    pub fn reverse(x: i32) -> i32 {
        let mut xx = x;
        let mut rx = 0i32;
        while xx != 0 {
            if let Some(t) = rx.checked_mul(10) {
                rx = t;
            } else {
                return 0;
            }
            if let Some(t) = rx.checked_add(xx % 10) {
                rx = t;
            } else {
                return 0;
            }
            xx = xx / 10;
        }
        rx
    }

    // 1108. Defanging an IP Address.
    // https://leetcode.com/problems/defanging-an-ip-address/
    pub fn defang_ip_addr(address: String) -> String {
        address.replace(".", "[.]")
    }

    // 1342. Number of Steps to Reduce a Number to Zero (Easy).
    // https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
    pub fn number_of_steps (num: i32) -> i32 {
        let mut cnt = 0;
        let mut n = num;
        while n != 0 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n -= 1;
            }
            cnt += 1;
        }
        cnt
    }

    // 206. Reverse Linked List.
    // https://leetcode.com/problems/reverse-linked-list/
    // Iterative solution.
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev_node = None;
        let mut current_node = head.take();

        while let Some(mut current_node_inner) = current_node.take() {
            let next_node = current_node_inner.next.take();
            current_node_inner.next = prev_node.take();
            prev_node = Some(current_node_inner);
            current_node = next_node;
        }

        prev_node.take()
    }

    // Recursive solution.
    pub fn reverse_list_v2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn solve(mut prev_node: Option<Box<ListNode>>, mut current_node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if let Some(mut current_node_inner) = current_node.take() {
                let next_node = current_node_inner.next.take();
                current_node_inner.next = prev_node.take();
                prev_node = Some(current_node_inner);
                current_node = next_node;
                solve(prev_node, current_node)
            } else {
                prev_node
            }
        }
        solve(None, head)
    }

    // 21. Merge Two Sorted Lists.
    // https://leetcode.com/problems/merge-two-sorted-lists/
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() { return l2; }
        if l2.is_none() { return l1; }

        let mut l3: Box<ListNode> = Box::new(ListNode::new(0)); // Sentinel node.
        let mut prev_node: &mut Box<ListNode> = &mut l3;

        while let Some(mut l1_inner) = l1 {
            if l2.is_none() {
                // Shortcut.
                prev_node.next = Some(l1_inner);
                break;
            }
            while let Some(mut l2_inner) = l2 {
                if l1_inner.val > l2_inner.val {
                    l2 = l2_inner.next.take();
                    prev_node.next = Some(l2_inner);
                    prev_node = prev_node.next.as_mut().unwrap();
                } else {
                    l2 = Some(l2_inner);
                    break;
                }
            }
            l1 = l1_inner.next.take();
            prev_node.next = Some(l1_inner);
            prev_node = prev_node.next.as_mut().unwrap();
        }
        if l2.is_some() {
            prev_node.next = l2;
        }
        l3.next.take()
    }

    // 20. Valid Parentheses.
    // https://leetcode.com/problems/valid-parentheses/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c)
            } else {
                if stack.is_empty() {
                    return false;
                }
                let o = stack.pop().unwrap();
                if o == '(' && c != ')' || o == '[' && c != ']' || o == '{' && c != '}' {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}