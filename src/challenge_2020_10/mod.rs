mod recent_counter;

#[cfg(test)]
mod tests;

use std::cmp::{max, min, Ordering};
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;

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
}