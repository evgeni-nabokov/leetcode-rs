use std::collections::HashMap;
use std::mem::replace;
use std::rc::Rc;
use std::cell::RefCell;

use crate::common::tree_node::TreeNode;
use crate::common::list_node::ListNode;

#[cfg(test)]
mod tests;

struct Solution;

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

    // 26. Remove Duplicates from Sorted Array.
    //https://leetcode.com/problems/remove-duplicates-from-sorted-array/
    // Time complexity: O(N).
    // Space complexityL O(1).
    pub fn remove_duplicates_from_sorted_array(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut k = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
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
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr: Option<Box<ListNode>> = head;
        let mut prev: Option<Box<ListNode>> = None;

        while let Some(mut curr_inner) = curr {
            let next = curr_inner.next;
            curr_inner.next = prev;
            prev = Some(curr_inner);
            curr = next;
        }

        prev
    }

    // Recursive solution.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn reverse_list_v2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn solve(curr: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if let Some(mut curr_inner) = curr {
                solve(replace(&mut curr_inner.next, prev), Some(curr_inner))
            } else {
                prev
            }
        }
        solve(head, None)
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

    // 724. Find Pivot Index.
    // https://leetcode.com/problems/find-pivot-index/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        let mut current_sum = 0;
        for i in 0..nums.len() {
            let left_sum = current_sum;
            current_sum += nums[i];
            let right_sum = total_sum - current_sum;
            if left_sum == right_sum {
                return i as i32;
            }
        }
        -1
    }

    // 1991. Find the Middle Index in Array.
    // https://leetcode.com/problems/find-the-middle-index-in-array/submissions/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        let mut current_sum = 0;
        for i in 0..nums.len() {
            let left_sum = current_sum;
            current_sum += nums[i];
            let right_sum = total_sum - current_sum;
            if left_sum == right_sum {
                return i as i32;
            }
        }
        -1
    }

    // 680. Valid Palindrome II
    // https://leetcode.com/problems/valid-palindrome-ii/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn valid_palindrome(s: String) -> bool {
        fn solve(sub_s: &[u8], errors: i32) -> bool {
            if sub_s.len() < 2 {
                return true;
            }

            let mut left = 0;
            let mut right = sub_s.len() - 1;
            while left <= right {
                if sub_s[left] != sub_s[right] {
                    if errors > 0 {
                        return false;
                    }

                    return solve(&sub_s[left + 1..=right], errors + 1) || solve(&sub_s[left..right], errors + 1);
                }
                left += 1;
                right -= 1;
            }

            true
        }

        solve(s.as_bytes(), 0)
    }

    // 1047. Remove All Adjacent Duplicates In String.
    // https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len());
        for b in s.as_bytes() {
            if let Some(last) = stack.last() {
                if last == b {
                    stack.pop();
                } else {
                    stack.push(*b);
                }
            } else {
                stack.push(*b);
            }
        }
        String::from_utf8(stack).unwrap()
    }

    // 415. Add Strings.
    // https://leetcode.com/problems/add-strings/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn add_strings(num_1: String, num_2: String) -> String {
        let mut digs_1 = num_1.bytes().rev().map(|c| c - '0' as u8).collect::<Vec<_>>();
        let mut digs_2 = num_2.bytes().rev().map(|c| c - '0' as u8).collect::<Vec<_>>();
        if digs_1.len() < digs_2.len() {
            std::mem::swap(&mut digs_1, &mut digs_2);
        }
        let mut res = Vec::with_capacity(std::cmp::max(digs_1.len(), digs_2.len()) + 1);
        let mut sum = 0;
        for (d_1, d_2) in digs_1.into_iter().zip(digs_2.into_iter().chain(std::iter::repeat(0))) {
            sum += d_1 + d_2;
            if sum > 9 {
                sum -= 10;
                res.push(sum);
                sum = 1;
            } else {
                res.push(sum);
                sum = 0;
            }
        }
        if sum > 0 {
            res.push(sum);
        }
        res.into_iter().rev().map(|d| (d + '0' as u8) as char).collect()
    }

    // 1588. Sum of All Odd Length Subarrays.
    // https://leetcode.com/problems/sum-of-all-odd-length-subarrays/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut prefix_sum = Vec::with_capacity(arr.len() + 1);
        prefix_sum.push(0);
        for i in 0..arr.len() {
            prefix_sum.push(prefix_sum[i] + arr[i])
        }

        let mut sum = 0;
        for start in 0..arr.len() {
            for end in (start..arr.len()).step_by(2) {
                sum += prefix_sum[end + 1] - prefix_sum[start];
            }
        }

        sum
    }

    // 543. Diameter of Binary Tree.
    // https://leetcode.com/problems/diameter-of-binary-tree/
    // Time complexity: O(N);
    // Space complexity: O(N);
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, curr_max: &mut i32) -> i32 {
            if let Some(node_inner) = node {
                let left_path = dfs(&node_inner.borrow().left, curr_max);
                let right_path = dfs(&node_inner.borrow().right, curr_max);
                *curr_max = (left_path + right_path).max(*curr_max);
                left_path.max(right_path) + 1
            } else {
                0
            }
        }

        let mut curr_max = 0;
        dfs(&root, &mut curr_max);
        curr_max
    }
}