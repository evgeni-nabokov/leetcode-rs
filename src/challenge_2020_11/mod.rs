#[cfg(test)]
mod tests;

use std::cmp::{max, min};

use crate::common::list_node::ListNode;
use crate::common::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

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

    // 1446. Consecutive Characters.
    // https://leetcode.com/problems/consecutive-characters/
    pub fn max_power(s: String) -> i32 {
        if s.len() == 1 { return 1; }
        let chars: Vec<char> = s.chars().collect();
        let mut curr_pow = 1;
        let mut max_pow = 0;
        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                curr_pow += 1
            } else {
                max_pow = max(max_pow, curr_pow);
                curr_pow = 1;
            }
        }
        max(max_pow, curr_pow)
    }

    // 310. Minimum Height Trees
    // https://leetcode.com/problems/minimum-height-trees/
    // Backtracking solution.
    // Not accepted - TLE.
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn backtrack(k: usize, adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, mut height: usize) -> usize {
            if visited[k] { return height; }
            height += 1;
            visited[k] = true;
            let mut res = 0;
            for i in 0..adj_list[k].len() {
                res = max(res, backtrack(adj_list[k][i], adj_list, visited, height));
            }
            res
        }

        let n = n as usize;
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n];
        for i in 0..edges.len() {
            adj_list[edges[i][0] as usize].push(edges[i][1] as usize);
            adj_list[edges[i][1] as usize].push(edges[i][0] as usize);
        }
        let mut heights = vec![0; n];
        let mut min_height = n;
        for i in 0..n {
            let mut visited: Vec<bool> = vec![false; n];
            heights[i] = backtrack(i, &adj_list, &mut visited, 0);
            min_height = min(min_height, heights[i]);
        }
        let mut res: Vec<i32> = Vec::new();
        for i in 0..n {
            if heights[i] == min_height {
                res.push(i as i32);
            }
        }
        res
    }

    // 563. Binary Tree Tilt.
    // https://leetcode.com/problems/binary-tree-tilt/
    // DFS solution.
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(node_inner) = node {
                let (left_sum, left_tilt) = dfs(&node_inner.borrow().left);
                let (right_sum, right_tilt) = dfs(&node_inner.borrow().right);
                (left_sum + right_sum + node_inner.borrow().val, left_tilt + right_tilt + (left_sum - right_sum).abs())
            } else {
                (0, 0)
            }
        }
        let (_, tilt) = dfs(&root);
        tilt
    }

    // 1099. Two Sum Less Than K.
    // https://leetcode.com/problems/two-sum-less-than-k/
    pub fn two_sum_less_than_k(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut res = -1;
        let mut i = 0;
        while nums[i] <= k {
            match nums[(i + 1)..].binary_search(&(k - nums[i] - 1)) {
                Ok(j) => return nums[i] + nums[i + 1 + j],
                Err(j) if j > 0 => res = max(res, nums[i] + nums[i + j]),
                _ => break,
            }
            i += 1;
        }
        res
    }

    pub fn two_sum_less_than_k_v2(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut res = -1;
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            if nums[left] + nums[right] < k {
                res = max(res, nums[left] + nums[right]);
                left += 1;
            } else {
                right -= 1;
            }
        }
        res
    }

    // 832. Flipping an Image.
    // https://leetcode.com/problems/flipping-an-image/
    pub fn flip_and_invert_image(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for r in 0..matrix.len() {
            matrix[r].reverse();
            for c in 0..matrix[r].len() {
                matrix[r][c] = if matrix[r][c] == 0 { 1 } else { 0 };
            }
        }
        matrix
    }
}
