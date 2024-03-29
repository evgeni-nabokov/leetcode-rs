#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::mem::replace;
use std::rc::Rc;

use crate::common::list_node::ListNode;
use crate::common::tree_node::TreeNode;

struct Solution;

impl Solution {
    // 1. Two Sum.
    // https://leetcode.com/problems/two-sum/
    // One-pass hash map method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let x = target - nums[i];
            if let Some(j) = map.get(&x) {
                return vec![i as i32, *j as i32];
            } else {
                map.insert(nums[i], i);
            }
        }
        vec![]
    }

    // Two-pass hash map method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            map.insert(nums[i], i);
        }

        for i in 0..nums.len() {
            let x = target - nums[i];
            match map.get(&x) {
                Some(j) if *j != i => {
                    return vec![i as i32, *j as i32];
                }
                _ => (),
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

    // 13. Roman to Integer.
    // https://leetcode.com/problems/roman-to-integer/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut prev_c = '#';
        for b in s.as_bytes() {
            let curr_c = *b as char;
            sum += match (prev_c, curr_c) {
                ('C', 'M') => 800,
                (_, 'M') => 1000,
                ('C', 'D') => 300,
                (_, 'D') => 500,
                ('X', 'C') => 80,
                (_, 'C') => 100,
                ('X', 'L') => 30,
                (_, 'L') => 50,
                ('I', 'X') => 8,
                (_, 'X') => 10,
                ('I', 'V') => 3,
                (_, 'V') => 5,
                (_, 'I') => 1,
                _ => unreachable!(),
            };
            prev_c = curr_c;
        }
        sum
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

    // 94. Binary Tree Inorder Traversal.
    // https://leetcode.com/problems/binary-tree-inorder-traversal/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(inner_node) = node {
                let borrowed_node = inner_node.borrow();
                dfs(&borrowed_node.left, res);
                res.push(borrowed_node.val);
                dfs(&borrowed_node.right, res);
            }
        }

        dfs(&root, &mut res);

        res
    }

    // 1108. Defanging an IP Address.
    // https://leetcode.com/problems/defanging-an-ip-address/
    pub fn defang_ip_addr(address: String) -> String {
        address.replace(".", "[.]")
    }

    // 1342. Number of Steps to Reduce a Number to Zero (Easy).
    // https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
    pub fn number_of_steps(num: i32) -> i32 {
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
        fn solve(
            curr: Option<Box<ListNode>>,
            prev: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
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
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }
        if l2.is_none() {
            return l1;
        }

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

                    return solve(&sub_s[left + 1..=right], errors + 1)
                        || solve(&sub_s[left..right], errors + 1);
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
        let mut digs_1 = num_1
            .bytes()
            .rev()
            .map(|c| c - '0' as u8)
            .collect::<Vec<_>>();
        let mut digs_2 = num_2
            .bytes()
            .rev()
            .map(|c| c - '0' as u8)
            .collect::<Vec<_>>();
        if digs_1.len() < digs_2.len() {
            std::mem::swap(&mut digs_1, &mut digs_2);
        }
        let mut res = Vec::with_capacity(std::cmp::max(digs_1.len(), digs_2.len()) + 1);
        let mut sum = 0;
        for (d_1, d_2) in digs_1
            .into_iter()
            .zip(digs_2.into_iter().chain(std::iter::repeat(0)))
        {
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
        res.into_iter()
            .rev()
            .map(|d| (d + '0' as u8) as char)
            .collect()
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

    // 1710. Maximum Units on a Truck.
    // https://leetcode.com/problems/maximum-units-on-a-truck/
    // Sort method.
    // Time complexity: O(LogN).
    // Space complexity: O(1).
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_unstable_by(|x, y| y[1].cmp(&x[1]));

        let mut unit_count = 0;
        let mut remaining_truck_size = truck_size;
        for bt in box_types {
            let box_count = remaining_truck_size.min(bt[0]);
            unit_count += box_count * bt[1];
            remaining_truck_size -= box_count;
            if remaining_truck_size == 0 {
                break;
            }
        }
        unit_count
    }

    // Max binary heap method.
    // Time complexity: O(LogN).
    // Space complexity: O(N).
    pub fn maximum_units_v2(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct BoxType {
            box_count: i32,
            unit_count: i32,
        }

        impl Ord for BoxType {
            fn cmp(&self, other: &Self) -> Ordering {
                other.unit_count.cmp(&self.unit_count)
            }
        }

        impl PartialOrd for BoxType {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(other.cmp(&self))
            }
        }

        let mut pq = BinaryHeap::from_iter(box_types.into_iter().map(|x| BoxType {
            box_count: x[0],
            unit_count: x[1],
        }));

        let mut unit_count = 0;
        let mut remaining_truck_size = truck_size;
        while !pq.is_empty() && remaining_truck_size > 0 {
            let top = pq.pop().unwrap();
            let box_count = remaining_truck_size.min(top.box_count);
            remaining_truck_size -= box_count;
            unit_count += box_count * top.unit_count;
        }

        unit_count
    }

    // 746. Min Cost Climbing Stairs
    // https://leetcode.com/problems/min-cost-climbing-stairs/
    // Recursive top-down DP method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        fn solve(cost: &[i32], i: usize, memo: &mut Vec<i32>) -> i32 {
            if i <= 1 {
                return 0;
            }
            if memo[i] >= 0 {
                return memo[i];
            }
            memo[i] = (cost[i - 1] + solve(&cost, i - 1, memo))
                .min(cost[i - 2] + solve(&cost, i - 2, memo));
            memo[i]
        }

        let mut memo = vec![-1; cost.len() + 1];
        solve(&cost, cost.len(), &mut memo);
        memo[cost.len()]
    }

    // Top-down DP iterative method. Constant space.
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn min_cost_climbing_stairs_v2(cost: Vec<i32>) -> i32 {
        let mut memo = vec![0, 0];

        for i in 2..=cost.len() {
            let tmp = memo[1];
            memo[1] = (cost[i - 1] + memo[1]).min(cost[i - 2] + memo[0]);
            memo[0] = tmp;
        }

        memo[1]
    }

    // Bottom-up DP iterative method.
    // Time complexity: O(N).
    // Space complexity: O(1).
    // pub fn min_cost_climbing_stairs_v3(cost: Vec<i32>) -> i32 {
    //     let mut memo = vec![0, 0];
    //
    // }

    // 268. Missing Number.
    // https://leetcode.com/problems/missing-number/
    // Two sums calculation method.
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let actual_sum: i32 = nums.iter().sum();
        let n = nums.len() as i32;
        let expected_sum = n * (n + 1) / 2;
        expected_sum - actual_sum
    }

    // XOR method.
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn missing_number_v2(nums: Vec<i32>) -> i32 {
        let mut res = nums.len() as i32;
        for i in 0..nums.len() {
            res ^= i as i32 ^ nums[i];
        }
        res
    }

    // 9. Palindrome Number.
    // https://leetcode.com/problems/palindrome-number/
    // Time complexity: O(LogN).
    // Space complexity: O(1).
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut reverted_x = 0;

        while x > reverted_x {
            reverted_x = 10 * reverted_x + x % 10;
            x /= 10;
        }

        x == reverted_x || x == reverted_x / 10
    }

    // 1217. Minimum Cost to Move Chips to The Same Position.
    // https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut even_count = 0;
        let mut odd_count = 0;

        for p in position {
            if p % 2 == 0 {
                even_count += 1;
            } else {
                odd_count += 1;
            }
        }

        even_count.min(odd_count)
    }

    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn min_cost_to_move_chips_v2(position: Vec<i32>) -> i32 {
        let odd_count = position.iter().fold(0, |odd_count, x| odd_count + *x % 2);
        odd_count.min(position.len() as i32 - odd_count)
    }

    // 937. Reorder Data in Log Files.
    // https://leetcode.com/problems/reorder-data-in-log-files/
    // Time complexity: O(N * LogN).
    // Space complexity: O(N).
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut split_logs = logs
            .iter()
            .map(|x| (x.split_ascii_whitespace().collect::<Vec<_>>(), x))
            .collect::<Vec<(Vec<_>, _)>>();

        split_logs.sort_by(|x, y| {
            let x_first_char = x.0[1].as_bytes()[0];
            let y_first_char = y.0[1].as_bytes()[0];

            match (x_first_char.is_ascii_digit(), y_first_char.is_ascii_digit()) {
                (true, true) => Ordering::Equal,
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (false, false) => {
                    let l = x.0.len().min(y.0.len());
                    for i in 1..l {
                        let ord = x.0[i].cmp(&y.0[i]);
                        if ord != Ordering::Equal {
                            return ord;
                        }
                    }

                    let ord = x.0.len().cmp(&y.0.len());
                    if ord != Ordering::Equal {
                        return ord;
                    }

                    x.0[0].cmp(&y.0[0])
                }
            }
        });

        split_logs.into_iter().map(|x| x.1.clone()).collect()
    }

    // 766. Toeplitz Matrix.
    // https://leetcode.com/problems/toeplitz-matrix/
    // Time complexity: O(M * N), where M, N - the dimensions of the matrix.
    // Space complexity: O(1).
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for r in 0..matrix.len() - 1 {
            for c in 0..matrix[0].len() - 1 {
                if matrix[r][c] != matrix[r + 1][c + 1] {
                    return false;
                }
            }
        }

        true
    }

    // 1200. Minimum Absolute Difference.
    // https://leetcode.com/problems/minimum-absolute-difference/
    // Time complexity: O(N * LogN).
    // Space complexity: O(1).
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();

        let mut min_diff = i32::MAX;
        for i in 1..arr.len() {
            min_diff = min_diff.min(arr[i] - arr[i - 1]);
        }

        let mut res = vec![];
        for i in 1..arr.len() {
            if min_diff == arr[i] - arr[i - 1] {
                res.push(vec![arr[i - 1], arr[i]]);
            }
        }

        res
    }

    // 257. Binary Tree Paths.
    // https://leetcode.com/problems/binary-tree-paths/
    // DFS method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn dfs(node: &TreeNode, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            path.push(node.val);

            match (&node.left, &node.right) {
                (None, None) => res.push(path.clone()),
                (Some(left), Some(right)) => {
                    dfs(&*left.borrow(), &mut path.clone(), res);
                    dfs(&*right.borrow(), path, res);
                }
                (Some(left), None) => dfs(&*left.borrow(), path, res),
                (None, Some(right)) => dfs(&*right.borrow(), path, res),
            }
        }

        if root.is_none() {
            return vec![];
        }

        let mut res = vec![];
        dfs(&root.as_ref().unwrap().borrow(), &mut vec![], &mut res);

        res.into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join("->")
            })
            .collect()
    }

    // 2169. Count Operations to Obtain Zero.
    // https://leetcode.com/problems/count-operations-to-obtain-zero/
    // Time complexity: O(LogN).
    // Space complexity: O(1).
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut count = 0;

        while num1 != 0 && num2 != 0 {
            if num1 >= num2 {
                count += num1 / num2;
                num1 %= num2;
            } else {
                count += num2 / num1;
                num2 %= num1;
            }
        }

        count
    }
}
