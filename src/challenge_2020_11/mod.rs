#[cfg(test)]
mod tests;

use crate::common::list_node::ListNode;

struct Solution;

impl Solution {
    // 252. Meeting Rooms.
    // https://leetcode.com/problems/meeting-rooms/
    // Solution with O(N*LogN) time and O(1) space.
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable();
        intervals.windows(2).all(|slice| slice[0][1] <= slice[1][0])
    }

    // 1290. Convert Binary Number in a Linked List to Integer.
    // https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
    // Solution with O(N) time and O(1) space.
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut bits: Vec<i32> = Vec::new();
        let mut node = head;
        while let Some(node_inner) = node {
            bits.push(node_inner.val);
            node = node_inner.next;
        }
        bits.into_iter().fold(0, |acc, b| (acc << 1) | b)
    }
}