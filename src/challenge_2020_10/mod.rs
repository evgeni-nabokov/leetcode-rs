mod recent_counter;
mod two_sum;

#[cfg(test)]
mod tests;

use std::cmp::{max, min, Ordering};
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;
use crate::common::list_node::ListNode;

struct Solution;

impl Solution {
    // 624. Maximum Distance in Arrays.
    // https://leetcode.com/problems/maximum-distance-in-arrays/
    // Single-scan solution.
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        // Even though the first array has the min and the max, its range (max - min) is never assigned to res.
        let mut res = 0;
        let (mut min_a, mut max_b) = (arrays[0][0], arrays[0][arrays[0].len() - 1]);
        for i in 1..arrays.len() {
            let (a, b) = (arrays[i][0], arrays[i][arrays[i].len() - 1]);
            res = max(res, max((min_a - b).abs(), (a - max_b).abs()));
            min_a = min(min_a, a);
            max_b = max(max_b, b);
        }
        res
    }

    // 532. K-diff Pairs in an Array.
    // https://leetcode.com/problems/k-diff-pairs-in-an-array/
    // Sorting solution.
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut count = 0;
        let mut left = 0;
        let mut right = 1;
        while left < nums.len() && right < nums.len() {
            if left == right || nums[right] - nums[left] < k {
                right += 1;
            } else if nums[right] - nums[left] > k {
                left += 1;
            } else {
                count += 1;
                left += 1;
                while left < nums.len() && nums[left] == nums[left - 1] {
                    left += 1;
                }
            }
        }
        count
    }

    // 1288. Remove Covered Intervals.
    // https://leetcode.com/problems/remove-covered-intervals/
    // Sorting solution.
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| match a[0].cmp(&b[0]) {
            Ordering::Equal => b[1].cmp(&a[1]),
            x => x
        });
        let mut res = 1;
        let mut a = intervals[0][0];
        let mut b = intervals[0][1];
        for i in 1..intervals.len() {
            let c = intervals[i][0];
            let d = intervals[i][1];
            if !(c >= a && d <= b) {
                a = intervals[i][0];
                b = intervals[i][1];
                res += 1;
            }
        }
        res
    }

    // 701. Insert into a Binary Search Tree.
    // https://leetcode.com/problems/insert-into-a-binary-search-tree/
    pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn new_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: None,
            })))
        }

        if root.is_none() {
            return new_node(val);
        }

        fn dfs(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
            if val > node.as_ref().unwrap().borrow().val {
                if node.as_ref().unwrap().borrow().right.is_none() {
                    node.as_mut().unwrap().borrow_mut().right = new_node(val);
                } else {
                    dfs(&mut node.as_mut().unwrap().borrow_mut().right, val);
                }
            } else {
                if node.as_ref().unwrap().borrow().left.is_none() {
                    node.as_mut().unwrap().borrow_mut().left = new_node(val);
                } else {
                    dfs(&mut node.as_mut().unwrap().borrow_mut().left, val);
                }
            }
        }

        dfs(&mut root, val);
        root
    }

    // 61. Rotate List.
    // https://leetcode.com/problems/rotate-list/
    pub fn rotate_right(head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }

        fn get_length(head: &Option<Box<ListNode>>) -> usize {
            let mut res = 0;
            let mut curr_node = head;
            while curr_node.is_some() {
                curr_node = &curr_node.as_ref().unwrap().next;
                res += 1;
            }
            res
        }

        let len = get_length(&head);
        if len == 1 { return head; }
        k %= len as i32;
        if k == 0 { return head; }

        fn split(mut head: Option<Box<ListNode>>, len: usize) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
            let mut curr_node = &mut head;
            for _ in 0..len {
                let curr_node_inner = curr_node.as_mut().unwrap();
                curr_node = &mut curr_node_inner.next;
            }
            let new_head = curr_node.take();
            (head, new_head)
        }

        fn concat(mut head_1: Option<Box<ListNode>>, head_2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut curr_node = &mut head_1;
            while curr_node.as_ref().unwrap().next.is_some() {
                curr_node = &mut curr_node.as_mut().unwrap().next;
            }

            curr_node.as_mut().unwrap().next = head_2;
            head_1
        }

        let (head_1, head_2) = split(head, len - k as usize);
        concat(head_2, head_1)
    }

    // 704. Binary Search.
    // https://leetcode.com/problems/binary-search/solution/
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => { return mid; },
                Ordering::Greater => { right = mid - 1; }
                Ordering::Less => { left = mid + 1; }
            }
        }
        -1
    }

    // 452. Minimum Number of Arrows to Burst Balloons.
    // https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 2 { return points.len() as i32; }
        points.sort_unstable_by_key(|x| x[1]);
        let mut res = 1;
        let mut b = points[0][1];
        for i in 1..points.len() {
            if points[i][0] > b {
                b = points[i][1];
                res += 1;
            }
        }
        res
    }
}