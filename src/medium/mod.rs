mod bst_iterator;
mod snapshot_array;
mod suggested_products;
#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::mem::swap;
use std::rc::Rc;
use std::str;

use crate::common::list_node::ListNode;
use crate::common::tree_node::TreeNode;

struct Solution;

impl Solution {
    // 2. Add Two Numbers.
    // https://leetcode.com/problems/add-two-numbers/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Calculates the length of a list.
        fn get_len(mut list: &Option<Box<ListNode>>) -> usize {
            let mut cnt = 0;
            while let Some(list_inner) = list {
                cnt += 1;
                list = &list_inner.next;
            }
            cnt
        }

        // Calculate lengths of both lists.
        let l1_len = get_len(&l1);
        let l2_len = get_len(&l2);

        // Make l1 pointing at the longest list.
        if l2_len > l1_len {
            std::mem::swap(&mut l1, &mut l2);
        }

        let mut curr = &mut l1;
        let mut sum = 0;
        // Calculate sum in-place - l1 will contain the result.
        while let Some(l1_inner) = curr {
            sum += l1_inner.val;
            if let Some(l2_inner) = l2 {
                sum += l2_inner.val;
                l2 = l2_inner.next;
            }
            if sum < 10 {
                l1_inner.val = sum;
                sum = 0;
            } else {
                l1_inner.val = sum - 10;
                sum = 1;
            }
            if l1_inner.next.is_none() && sum > 0 {
                l1_inner.next = Some(Box::new(ListNode::new(sum)));
                break;
            }
            curr = &mut l1_inner.next;
        }

        l1
    }

    // 63. Unique Paths II.
    // https://leetcode.com/problems/unique-paths-ii/
    // Bottom-up DP iterative method.
    // Time complexity: O(MxN).
    // Space complexity: O(MxN).
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; obstacle_grid[0].len() + 1]; obstacle_grid.len() + 1];

        dp[0][1] = 1;

        for r in 1..dp.len() {
            for c in 1..dp[0].len() {
                if obstacle_grid[r - 1][c - 1] == 0 {
                    dp[r][c] = dp[r - 1][c] + dp[r][c - 1];
                }
            }
        }

        dp[obstacle_grid.len()][obstacle_grid[0].len()]
    }

    // 71. Simplify Path.
    // https://leetcode.com/problems/simplify-path/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn simplify_path(path: String) -> String {
        let mut res = Vec::with_capacity(path.len());
        let mut segment = vec![];
        for &b in path.as_bytes().into_iter().chain([b'/'].iter()) {
            if segment.is_empty() {
                segment.push(b);
                continue;
            }

            if b == b'/' {
                match segment.as_slice() {
                    [b'/'] | [b'/', b'.'] => segment.clear(),
                    [b'/', b'.', b'.'] => {
                        while !res.is_empty() && res.pop().unwrap() != b'/' {}
                        segment.clear();
                    }
                    _ => res.append(&mut segment),
                }
            }
            segment.push(b);
        }

        if res.is_empty() {
            res.push(b'/');
        }

        res.into_iter().map(|x| x as char).collect()
    }

    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn simplify_path_v2(path: String) -> String {
        let mut res = Vec::new();

        for segment in path.split('/') {
            if segment.is_empty() || segment == "." {
                continue;
            }

            if segment == ".." {
                res.pop();
                continue;
            }

            res.push(segment);
        }

        ['/' as char]
            .into_iter()
            .chain(res.join("/").chars())
            .collect()

        // Alternative concatenation.
        // let mut path = String::from("/");
        // path.extend(res.join("/").chars());
        // path
    }

    // 274. H-Index
    // https://leetcode.com/problems/h-index/
    // Time complexity: O(N * LogN).
    // Space complexity: O(1).
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        if citations.is_empty() {
            return 0;
        }
        citations.sort_unstable();
        let l = citations.len() as i32;
        let mut left = 0;
        let mut right = l - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let h = l - mid;
            match citations[mid as usize].cmp(&h) {
                Ordering::Equal => return h,
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
            }
        }
        l - left
    }

    // 1325. Delete Leaves With a Given Value.
    // https://leetcode.com/problems/delete-leaves-with-a-given-value/
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(some) = root {
            let left = Solution::remove_leaf_nodes(some.borrow_mut().left.clone(), target);
            let right = Solution::remove_leaf_nodes(some.borrow_mut().right.clone(), target);
            if left.is_none() && right.is_none() && some.borrow_mut().val == target {
                None
            } else {
                some.borrow_mut().left = left.clone();
                some.borrow_mut().right = right.clone();
                Some(some)
            }
        } else {
            None
        }
    }

    // 102. Binary Tree Level Order Traversal.
    // https://leetcode.com/problems/binary-tree-level-order-traversal/
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<Vec<i32>>, level: usize) {
            if node.is_none() {
                return;
            }
            if level == levels.len() {
                levels.push(vec![]);
            }
            levels[level].push(RefCell::borrow(node.as_ref().unwrap()).val);
            dfs(
                RefCell::borrow(node.as_ref().unwrap()).left.clone(),
                levels,
                level + 1,
            );
            dfs(
                RefCell::borrow(node.as_ref().unwrap()).right.clone(),
                levels,
                level + 1,
            );
        }

        let mut levels: Vec<Vec<i32>> = Vec::new();
        dfs(root, &mut levels, 0);
        levels
    }

    // 889. Construct Binary Tree from Preorder and Postorder Traversal.
    // https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
    pub fn build_tree(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_bt(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || postorder.is_empty() {
                return None;
            }
            let mut val = preorder[0];
            let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            if preorder.len() == 1 {
                return node;
            }
            val = preorder[1];
            let i = postorder.iter().position(|x| *x == val).unwrap();
            node.as_ref().unwrap().borrow_mut().left =
                build_bt(&preorder[1..=1 + i], &postorder[..=i]);
            node.as_ref().unwrap().borrow_mut().right =
                build_bt(&preorder[i + 2..], &postorder[i + 1..postorder.len() - 1]);
            node
        }

        build_bt(&preorder, &postorder)
    }

    // 105. Construct Binary Tree from Preorder and Inorder Traversal.
    // https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
    pub fn build_tree_ii(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_bt(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || inorder.is_empty() {
                return None;
            }
            let val = preorder[0];
            let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            if preorder.len() == 1 {
                return node;
            }
            let i = inorder.iter().position(|x| *x == val).unwrap();
            node.as_ref().unwrap().borrow_mut().left = build_bt(&preorder[1..i + 1], &inorder[..i]);
            node.as_ref().unwrap().borrow_mut().right =
                build_bt(&preorder[i + 1..], &inorder[i + 1..]);
            node
        }

        build_bt(&preorder, &inorder)
    }

    // 39. Combination Sum.
    // https://leetcode.com/problems/combination-sum/
    // Backtracking method.
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        fn backtrack(candidates: &Vec<i32>, target: i32, start: usize) -> Vec<Vec<i32>> {
            let mut res: Vec<Vec<i32>> = vec![vec![]; 0];
            for i in start..candidates.len() {
                match candidates[i].cmp(&target) {
                    Ordering::Equal => {
                        res.push(vec![candidates[i]]);
                        break;
                    }
                    Ordering::Less => {
                        for mut v in backtrack(candidates, target - candidates[i], i) {
                            v.push(candidates[i]);
                            res.push(v);
                        }
                    }
                    _ => break,
                }
            }
            res
        }

        backtrack(&candidates, target, 0)
    }

    // 40. Combination Sum II.
    // https://leetcode.com/problems/combination-sum-ii/
    pub fn combination_sum_ii(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        fn backtrack(candidates: &Vec<i32>, target: i32, start: usize) -> Vec<Vec<i32>> {
            let mut res: Vec<Vec<i32>> = vec![vec![]; 0];
            for i in start..candidates.len() {
                match candidates[i].cmp(&target) {
                    Ordering::Equal => {
                        res.push(vec![candidates[i]]);
                        break;
                    }
                    Ordering::Less if i < candidates.len() - 1 => {
                        // Extra check to eliminate duplicate sets.
                        // We don't need to analyze sets with the same number at the same position,
                        // because we analyzed them at previous iteration.
                        if i > start && candidates[i - 1] == candidates[i] {
                            continue;
                        }

                        for mut v in backtrack(candidates, target - candidates[i], i + 1) {
                            v.push(candidates[i]);
                            res.push(v);
                        }
                    }
                    _ => break,
                }
            }
            res
        }

        backtrack(&candidates, target, 0)
    }

    // 322. Coin Change.
    // https://leetcode.com/problems/coin-change/
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut dp: Vec<i32> = vec![i32::MAX; (amount + 1) as usize];
        dp[0] = 0;
        for a in 1..=amount {
            for c in &coins {
                if a - c >= 0 {
                    let p = dp[(a - c) as usize];
                    dp[a as usize] =
                        dp[a as usize].min(if p == i32::MAX { i32::MAX } else { p + 1 });
                }
            }
        }
        let res = dp[amount as usize];
        if res == i32::MAX {
            -1
        } else {
            res
        }
    }

    // 8. String to Integer (atoi).
    // https://leetcode.com/problems/string-to-integer-atoi/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn my_atoi(s: String) -> i32 {
        let chars: Vec<char> = s.chars().skip_while(|x| x.is_ascii_whitespace()).collect();
        if chars.is_empty() {
            return 0;
        }
        let mut sign = 1;
        let mut skip = 0;
        if chars[0] == '-' {
            sign *= -1;
            skip += 1;
        } else if chars[0] == '+' {
            skip += 1;
        }

        let digits: Vec<i32> = chars
            .into_iter()
            .skip(skip)
            .skip_while(|&x| x == '0')
            .take_while(|x| x.is_digit(10))
            .map(|x| match x {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => unreachable!(),
            })
            .collect();

        fn get_min_max(sign: i32) -> i32 {
            if sign > 0 {
                i32::MAX
            } else {
                i32::MIN
            }
        }

        let mut res: i32 = 0;
        for d in digits.into_iter() {
            if let Some(a) = res.checked_mul(10) {
                if let Some(b) = a.checked_add(d) {
                    res = b;
                } else {
                    return get_min_max(sign);
                }
            } else {
                return get_min_max(sign);
            }
        }
        sign * res
    }

    // 186. Reverse Words in a String II.
    // https://leetcode.com/problems/reverse-words-in-a-string-ii/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn reverse_words_ii(s: &mut Vec<char>) {
        s.reverse();
        let mut start = 0;
        for end in 1..=s.len() {
            if end == s.len() || s[end] == ' ' {
                s[start..end].reverse();
                start = end + 1;
            }
        }
    }

    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn reverse_words_ii_v2(s: &mut Vec<char>) {
        s.reverse();
        for word in s.split_mut(|&x| x == ' ') {
            word.reverse();
        }
    }

    // 98. Validate Binary Search Tree.
    // https://leetcode.com/problems/validate-binary-search-tree/
    // Preorder traversal.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            lower: Option<i32>,
            upper: Option<i32>,
        ) -> bool {
            if let Some(node_inner) = node {
                let val = node_inner.borrow().val;
                if (lower.is_some() && val <= lower.unwrap())
                    || upper.is_some() && (val >= upper.unwrap())
                {
                    false
                } else {
                    dfs(&node_inner.borrow().left, lower, Some(val))
                        && dfs(&node_inner.borrow().right, Some(val), upper)
                }
            } else {
                true
            }
        }

        dfs(&root, None, None)
    }

    // Inorder traversal.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn is_valid_bst_v2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, prev: Option<i32>) -> (bool, Option<i32>) {
            if let Some(node_inner) = node {
                let (is_valid, prev) = dfs(&node_inner.borrow().left, prev);
                if is_valid {
                    let val = node_inner.borrow().val;
                    if prev.is_some() && val <= prev.unwrap() {
                        (false, prev)
                    } else {
                        dfs(&node_inner.borrow().right, Some(val))
                    }
                } else {
                    (false, prev)
                }
            } else {
                (true, prev)
            }
        }

        dfs(&root, None).0
    }

    // 36. Valid Sudoku.
    // https://leetcode.com/problems/valid-sudoku/
    // Time complexity: O(1).
    // Space complexity: O(1).
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut digits = vec![false; 10];

        fn get_digit(c: char) -> usize {
            match c {
                '.' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => unreachable!(),
            }
        }

        fn reset_digits(digits: &mut Vec<bool>) {
            for i in 1..digits.len() {
                digits[i] = false;
            }
        }

        for row in 0..9 {
            for col in 0..9 {
                let d = get_digit(board[row][col]);
                if d > 0 && digits[d] {
                    return false;
                }
                digits[d] = true;
            }
            reset_digits(&mut digits);
        }

        for col in 0..9 {
            for row in 0..9 {
                let d = get_digit(board[row][col]);
                if d > 0 && digits[d] {
                    return false;
                }
                digits[d] = true;
            }
            reset_digits(&mut digits);
        }

        for block in 0..9 {
            let base_row = (block / 3) * 3;
            let base_col = (block % 3) * 3;

            for row in 0..3 {
                for col in 0..3 {
                    let d = get_digit(board[base_row + row][base_col + col]);
                    if d > 0 && digits[d] {
                        return false;
                    }
                    digits[d] = true;
                }
            }
            reset_digits(&mut digits);
        }

        true
    }

    // 5. Longest Palindromic Substring.
    // https://leetcode.com/problems/longest-palindromic-substring/
    // Time complexity: O(N^2).
    // Space complexity: O(1).
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        fn expand(chars: &Vec<char>, mut left: i32, mut right: i32) -> usize {
            while left >= 0
                && (right as usize) < chars.len()
                && chars[left as usize] == chars[right as usize]
            {
                left -= 1;
                right += 1;
            }
            (right - left - 1) as usize
        }

        let chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = 0;
        for i in 0..chars.len() - 1 {
            let len =
                expand(&chars, i as i32, i as i32).max(expand(&chars, i as i32, i as i32 + 1));
            if len > end - start {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }

        chars[start..=end].iter().collect()
    }

    // 1465. Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts.
    // https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/
    // Time complexity: O(N), where N - max(horizontal_cuts, vertical_cuts).
    // Space complexity: O(1).
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.sort_unstable();
        vertical_cuts.sort_unstable();

        let mut cut_h = horizontal_cuts[0];
        let mut cut_w = vertical_cuts[0];
        for i in 1..horizontal_cuts.len() {
            cut_h = cut_h.max(horizontal_cuts[i] - horizontal_cuts[i - 1]);
        }
        for i in 1..vertical_cuts.len() {
            cut_w = cut_w.max(vertical_cuts[i] - vertical_cuts[i - 1]);
        }
        cut_h = cut_h.max(h - horizontal_cuts.last().unwrap());
        cut_w = cut_w.max(w - vertical_cuts.last().unwrap());

        let m = 1000_000_007;
        (cut_h.rem_euclid(m) as u64 * cut_w.rem_euclid(m) as u64).rem_euclid(m as u64) as i32
    }

    // 763. Partition Labels.
    // https://leetcode.com/problems/partition-labels/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn partition_labels(s: String) -> Vec<i32> {
        let chars: Vec<char> = s.chars().collect();

        let mut last: Vec<usize> = vec![0; 26];
        for i in 0..chars.len() {
            let char_idx = chars[i] as usize - 97;
            last[char_idx] = last[char_idx].max(i);
        }

        let mut res: Vec<i32> = Vec::with_capacity(s.len());
        let mut start = 0;
        let mut end = 0;
        for i in 0..chars.len() {
            let char_idx = chars[i] as usize - 97;
            end = end.max(last[char_idx]);
            if i == end {
                let len = end - start + 1;
                res.push(len as i32);
                start = i + 1;
            }
        }

        res
    }

    // 797. All Paths From Source to Target.
    // https://leetcode.com/problems/all-paths-from-source-to-target/
    // Time complexity: O(2^N * N).
    // Space complexity: O(2^N * N).
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn bt(v: usize, graph: &Vec<Vec<i32>>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if v == graph.len() - 1 {
                res.push(path.to_vec());
                return;
            }
            for i in 0..graph[v].len() {
                path.push(graph[v][i]);
                bt(graph[v][i] as usize, graph, path, res);
                path.pop();
            }
        }

        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = vec![0];
        bt(0, &graph, &mut path, &mut res);
        res
    }

    // 236. Lowest Common Ancestor of a Binary Tree.
    // https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn solve(
            node: Option<Rc<RefCell<TreeNode>>>,
            p_val: i32,
            q_val: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node_inner) = node {
                let val = node_inner.borrow().val;
                if val == p_val || val == q_val {
                    Some(node_inner)
                } else {
                    let left = solve(node_inner.borrow_mut().left.take(), p_val, q_val);
                    let right = solve(node_inner.borrow_mut().right.take(), p_val, q_val);
                    if left.is_some() && right.is_some() {
                        Some(node_inner)
                    } else if left.is_some() {
                        left
                    } else if right.is_some() {
                        right
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        }

        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;

        solve(root, p_val, q_val).take()
    }

    // 1209. Remove All Adjacent Duplicates in String II.
    // https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string-ii/
    // Count stack method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack = Vec::with_capacity(s.len());
        let mut res_bytes = Vec::with_capacity(s.len());
        for b in s.as_bytes() {
            if let Some(last) = res_bytes.last() {
                if b == last {
                    let new_count = stack.pop().unwrap() + 1;
                    if new_count == k {
                        res_bytes.truncate(res_bytes.len() + 1 - k as usize);
                    } else {
                        res_bytes.push(*b);
                        stack.push(new_count);
                    }
                } else {
                    res_bytes.push(*b);
                    stack.push(1);
                }
            } else {
                res_bytes.push(*b);
                stack.push(1);
            }
        }
        String::from_utf8(res_bytes).unwrap()
    }

    // Two pointers method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn remove_duplicates_v2(s: String, k: i32) -> String {
        let mut bytes = s.as_bytes().to_vec();
        let mut count: Vec<i32> = vec![0; bytes.len()];
        let mut fast = 0;
        let mut slow = 0;
        while fast < bytes.len() {
            bytes[slow] = bytes[fast];
            count[slow] = if slow > 0 && bytes[slow - 1] == bytes[fast] {
                count[slow - 1] + 1
            } else {
                1
            };
            slow += 1;
            fast += 1;
            // Check count after increment to avoid subtract with overflow.
            if count[slow - 1] == k {
                slow -= k as usize;
            }
        }
        bytes.truncate(slow);
        String::from_utf8(bytes).unwrap()
    }

    // 215. Kth Largest Element in an Array.
    // https://leetcode.com/problems/kth-largest-element-in-an-array/
    // Quickselect method.
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
            let pivot_num = nums[left];
            let mut i = left + 1;
            let mut j = right;
            loop {
                while nums[i] < pivot_num {
                    if i == right {
                        break;
                    }
                    i += 1;
                }
                while nums[j] > pivot_num {
                    if j == left {
                        break;
                    }
                    j -= 1;
                }
                if i >= j {
                    break;
                }
                nums.swap(i, j);
                i += 1;
                j -= 1;
            }
            nums.swap(left, j);
            j
        }

        let find = nums.len() - k as usize;
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let pivot = partition(&mut nums, low, high);
            if pivot > find {
                high = pivot - 1;
            } else if pivot < find {
                low = pivot + 1;
            } else {
                break;
            }
        }
        nums[find]
    }

    // Max binary heap method.
    // Time complexity: O(N + K * LogN).
    // Space complexity: O(1).
    pub fn find_kth_largest_v2(mut nums: Vec<i32>, k: i32) -> i32 {
        fn heapify(nums: &mut [i32]) {
            for i in (1..=nums.len() / 2).rev() {
                move_down(nums, i);
            }
        }

        fn move_down(nums: &mut [i32], mut i: usize) {
            while 2 * i <= nums.len() {
                let mut left = 2 * i;
                let right = left + 1;
                if left < nums.len() && less(nums, left, right) {
                    left += 1;
                }
                if less(nums, left, i) {
                    break;
                }
                nums.swap(left - 1, i - 1);
                i = left;
            }
        }

        fn less(nums: &[i32], a: usize, b: usize) -> bool {
            nums[a - 1] < nums[b - 1]
        }

        heapify(&mut nums);

        for _ in 0..(k - 1) {
            nums[0] = nums.pop().unwrap();
            move_down(&mut nums, 1);
        }

        nums[0]
    }

    // Min binary heap method.
    // Time complexity: O(N * LogK).
    // Space complexity: O(K).
    pub fn find_kth_largest_v3(nums: Vec<i32>, k: i32) -> i32 {
        let mut pq = BinaryHeap::with_capacity(k as usize + 1);
        for i in 0..nums.len() {
            pq.push(Reverse(nums[i]));
            if pq.len() > k as usize {
                pq.pop();
            }
        }

        pq.pop().unwrap().0
    }

    // Sort method.
    // Time complexity: O(N * LogN).
    // Space complexity: O(1).
    pub fn find_kth_largest_v4(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums[nums.len() - k as usize]
    }

    // 1448. Count Good Nodes in Binary Tree.
    // https://leetcode.com/problems/count-good-nodes-in-binary-tree/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut curr_max: i32) -> i32 {
            if let Some(node_inner) = node {
                let val = node_inner.borrow().val;
                let mut count = 0;
                if val >= curr_max {
                    count += 1;
                    curr_max = val;
                }
                count
                    + dfs(&node_inner.borrow().left, curr_max)
                    + dfs(&node_inner.borrow().right, curr_max)
            } else {
                0
            }
        }

        dfs(&root, i32::MIN)
    }

    // 99. Recover Binary Search Tree.
    // https://leetcode.com/problems/recover-binary-search-tree/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = Vec::new();
        let mut curr = root.clone();
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
        let mut x: Option<Rc<RefCell<TreeNode>>> = None;
        let mut y: Option<Rc<RefCell<TreeNode>>> = None;
        while !stack.is_empty() || curr.is_some() {
            while curr.is_some() {
                stack.push(curr.clone());
                let left = curr.as_mut().unwrap().borrow_mut().left.clone();
                curr = left;
            }
            curr = stack.pop().unwrap();
            if prev.is_some()
                && curr.as_ref().unwrap().borrow().val < prev.as_ref().unwrap().borrow().val
            {
                y = curr.clone();
                if x.is_none() {
                    x = prev.clone();
                } else {
                    break;
                }
            }

            prev = curr.clone();
            let right = curr.as_mut().unwrap().borrow_mut().right.clone();
            curr = right;
        }

        swap(
            &mut x.as_mut().unwrap().borrow_mut().val,
            &mut y.as_mut().unwrap().borrow_mut().val,
        );
    }

    // 91. Decode Ways.
    // https://leetcode.com/problems/decode-ways/
    // Recursive DP method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn num_decodings(s: String) -> i32 {
        fn solve(i: usize, bytes: &[u8], memo: &mut HashMap<usize, i32>) -> i32 {
            if i == bytes.len() {
                return 1;
            }

            if bytes[i] == 48 {
                return 0;
            }

            if i == bytes.len() - 1 {
                return 1;
            }

            if let Some(c) = memo.get(&i) {
                return *c;
            }

            let mut res = solve(i + 1, bytes, memo);

            let n = (bytes[i] - 48) * 10 + bytes[i + 1] - 48;
            if n <= 26 {
                res += solve(i + 2, bytes, memo);
            }

            memo.insert(i, res);

            res
        }

        solve(0, s.as_bytes(), &mut HashMap::new())
    }

    // 337. House Robber III.
    // https://leetcode.com/problems/house-robber-iii/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(inner_node) = node {
                let (left_rob, left_not_rob) = dfs(&inner_node.borrow().left);
                let (right_rob, right_not_rob) = dfs(&inner_node.borrow().right);

                // If we rob this node, we cannot rob its children.
                let rob = inner_node.borrow().val + left_not_rob + right_not_rob;

                // Else we could choose to either rob its children or not.
                let not_rob = left_rob.max(left_not_rob) + right_rob.max(right_not_rob);

                (rob, not_rob)
            } else {
                (0, 0)
            }
        }

        let (rob, not_rob) = dfs(&root);

        rob.max(not_rob)
    }

    // 443. String Compression
    // https://leetcode.com/problems/string-compression/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() <= 1 {
            return chars.len() as _;
        }

        fn compress_group(
            chars: &mut Vec<char>,
            mut start: usize,
            mut size: u32,
            c: char,
        ) -> usize {
            chars[start] = c;
            start += 1;
            let mut idx = 0;
            if size > 1 {
                while size > 0 {
                    chars[start + idx] = std::char::from_digit(size % 10, 10).unwrap();
                    size /= 10;
                    idx += 1;
                }
                chars[start..start + idx].reverse();
            }

            start + idx as usize
        }

        let mut gr_start = 0;
        let mut cnt = 1;

        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                cnt += 1;
            } else {
                gr_start = compress_group(chars, gr_start, cnt, chars[i - 1]);
                cnt = 1;
            }
        }

        compress_group(chars, gr_start, cnt, *chars.last().unwrap()) as _
    }

    // 1306. Jump Game III.
    // https://leetcode.com/problems/jump-game-iii/
    // DFS method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
        fn dfs(arr: &mut [i32], i: i32) -> bool {
            if i < 0 || i >= arr.len() as i32 || arr[i as usize] < 0 {
                return false;
            }

            let ui = i as usize;
            if arr[ui] == 0 {
                return true;
            }

            arr[ui] *= -1;
            dfs(arr, i - arr[ui]) || dfs(arr, i + arr[ui])
        }

        dfs(&mut arr, start)
    }

    // BFS method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn can_reach_v2(mut arr: Vec<i32>, start: i32) -> bool {
        let mut queue = vec![start];
        while !queue.is_empty() {
            let i = queue.pop().unwrap();
            let ui = i as usize;
            if arr[ui] == 0 {
                return true;
            }

            if arr[ui] < 0 {
                continue;
            }

            if i - arr[ui] >= 0 {
                queue.push(i - arr[ui]);
            }

            if i + arr[ui] < arr.len() as i32 {
                queue.push(i + arr[ui]);
            }

            arr[ui] *= -1;
        }

        false
    }

    // 790. Domino and Tromino Tiling.
    // https://leetcode.com/problems/domino-and-tromino-tiling/
    // DP bottom-up method.
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn num_tilings(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let modulo = 1000_000_007u64;
        let mut curr_f = 2u64;
        let mut prev_f = 1u64;
        let mut curr_p = 1u64;

        for _ in 3..=n {
            let tmp = curr_f;
            curr_f = (curr_f + prev_f + 2 * curr_p) % modulo;
            curr_p = (curr_p + prev_f) % modulo;
            prev_f = tmp;
        }

        return curr_f as i32;
    }

    // DP top-down method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn num_tilings_v2(n: i32) -> i32 {
        let mut f_memo = HashMap::new();
        let mut p_memo = HashMap::new();

        fn f(n: i32, f_memo: &mut HashMap<i32, u64>, p_memo: &mut HashMap<i32, u64>) -> u64 {
            match n {
                1 => 1,
                2 => 2,
                _ => {
                    if f_memo.contains_key(&n) {
                        *f_memo.get(&n).unwrap()
                    } else {
                        let new_f = (f(n - 1, f_memo, p_memo)
                            + f(n - 2, f_memo, p_memo)
                            + 2 * p(n - 1, f_memo, p_memo))
                            % 1000_000_007;
                        f_memo.insert(n, new_f);
                        new_f
                    }
                }
            }
        }

        fn p(n: i32, f_memo: &mut HashMap<i32, u64>, p_memo: &mut HashMap<i32, u64>) -> u64 {
            match n {
                2 => 1,
                _ => {
                    if p_memo.contains_key(&n) {
                        *p_memo.get(&n).unwrap()
                    } else {
                        let new_p =
                            (f(n - 2, f_memo, p_memo) + p(n - 1, f_memo, p_memo)) % 1000_000_007;
                        p_memo.insert(n, new_p);
                        new_p
                    }
                }
            }
        }

        f(n, &mut f_memo, &mut p_memo) as i32
    }

    // 1423. Maximum Points You Can Obtain from Cards.
    // https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/
    // Time complexity: O(N), where N is number of cards to draw.
    // Space complexity: O(1).
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut sum: i32 = card_points.iter().take(k).sum();
        if k == card_points.len() {
            return sum;
        }

        let mut max_sum = sum;
        for i in 0..k {
            sum += card_points[card_points.len() - i - 1] - card_points[k - i - 1];
            max_sum = max_sum.max(sum);
        }
        max_sum
    }

    // 416. Partition Equal Subset Sum.
    // https://leetcode.com/problems/partition-equal-subset-sum/
    // Recursive DP method.
    // Time complexity: O(N * M) ?
    // Space complexity: O(N * M) ?
    // Where N is the number of array elements, M - is the subSetSum (half of the sum of the numbers).
    pub fn can_partition(nums: Vec<i32>) -> bool {
        fn solve(nums: &[i32], i: usize, target_sum: i32, memo: &mut [Vec<Option<bool>>]) -> bool {
            if i == nums.len() || target_sum < 0 {
                return false;
            }

            if target_sum == 0 {
                return true;
            }

            if let Some(res) = memo[i][target_sum as usize] {
                return res;
            }

            let res = solve(nums, i + 1, target_sum, memo)
                || solve(nums, i + 1, target_sum - nums[i], memo);

            memo[i][target_sum as usize] = Some(res);
            res
        }

        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }

        let target_sum = sum / 2;
        let mut memo = vec![vec![None; target_sum as usize + 1]; nums.len()];
        solve(&nums, 0, target_sum, &mut memo)
    }

    // 93. Restore IP Addresses
    // https://leetcode.com/problems/restore-ip-addresses/
    // Backtracking method.
    // Time complexity: O(1).
    // Space complexity: O(1).
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn is_valid(bytes: &[u8]) -> bool {
            if bytes.len() == 1 {
                return true;
            }

            if bytes.is_empty() || bytes.len() > 3 || bytes[0] == b'0' {
                return false;
            }

            bytes.iter().fold(0, |num, d| num * 10 + (d - b'0') as u16) <= 255
        }

        fn build_addr(groups: &[&[u8]]) -> String {
            groups
                .into_iter()
                .map(|x| str::from_utf8(x).unwrap())
                .collect::<Vec<_>>()
                .join(".")
        }

        fn bt<'a>(
            bytes: &'a [u8],
            start: usize,
            groups: &mut Vec<&'a [u8]>,
            res: &mut Vec<String>,
        ) {
            if groups.len() == 3 {
                let group = &bytes[start..];
                if is_valid(group) {
                    groups.push(group);
                    res.push(build_addr(groups));
                    groups.pop();
                }
            }

            let end = bytes.len().min(start + 3);
            for i in start..end {
                let group = &bytes[start..=i];
                if is_valid(group) {
                    groups.push(group);
                    bt(bytes, i + 1, groups, res);
                    groups.pop();
                }
            }
        }

        let mut res = vec![];
        let mut groups = vec![];
        bt(s.as_bytes(), 0, &mut groups, &mut res);

        res
    }

    // 300. Longest Increasing Subsequence.
    // https://leetcode.com/problems/longest-increasing-subsequence/
    // Patience sort and binary search method.
    // Time complexity: O(N * LogN).
    // Space complexity: O(N).
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sub = vec![nums[0]];

        for i in 1..nums.len() {
            match nums[i].cmp(sub.last().unwrap()) {
                Ordering::Greater => sub.push(nums[i]),
                Ordering::Less => {
                    match sub.binary_search(&nums[i]) {
                        Err(j) => sub[j] = nums[i],
                        _ => (),
                    };
                }
                _ => (),
            }
        }

        sub.len() as _
    }

    // Iterative DP method.
    // Time complexity: O(N^2).
    // Space complexity: O(N).
    pub fn length_of_lis_v2(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut max_len = 1;
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            max_len = max_len.max(dp[i]);
        }

        max_len
    }

    // 199. Binary Tree Right Side View.
    // https://leetcode.com/problems/binary-tree-right-side-view/
    // BFS (right to left) method.
    // Time complexity: O(N).
    // Space complexity: O(D), where D is a tree diameter.
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            res.push(queue.front().as_ref().unwrap().borrow().val);
            let len = queue.len();
            for _ in 0..len {
                let curr = queue.pop_front().unwrap();
                if curr.borrow().right.is_some() {
                    queue.push_back(curr.borrow_mut().right.take().unwrap());
                }
                if curr.borrow().left.is_some() {
                    queue.push_back(curr.borrow_mut().left.take().unwrap());
                }
            }
        }

        res
    }

    // DFS (right to left) method.
    // Time complexity: O(N).
    // Space complexity: O(D), where D is a tree diameter.
    pub fn right_side_view_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<i32>) {
            if node.is_none() {
                return;
            }

            if res.len() == level {
                res.push(node.as_ref().unwrap().borrow().val);
            }

            dfs(&node.as_ref().unwrap().borrow().right, level + 1, res);
            dfs(&node.as_ref().unwrap().borrow().left, level + 1, res);
        }

        let mut res = Vec::new();
        dfs(&root, 0, &mut res);
        res
    }

    // 739. Daily Temperatures.
    // https://leetcode.com/problems/daily-temperatures/
    // Monotonic stack method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn daily_temperatures(temps: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temps.len()];
        let mut stack = Vec::with_capacity(temps.len());

        for i in 0..temps.len() {
            while !stack.is_empty() && temps[*stack.last().unwrap()] < temps[i] {
                let j = stack.pop().unwrap();
                res[j] = (i - j) as i32;
            }
            stack.push(i);
        }

        res
    }

    // Backward traversal method.
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn daily_temperatures_v2(temps: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temps.len()];
        let mut hottest_temp = 0;
        for i in (0..temps.len()).rev() {
            if temps[i] >= hottest_temp {
                hottest_temp = temps[i];
            } else {
                let mut day = i + 1;
                while temps[i] >= temps[day] {
                    day = day + res[day] as usize;
                }
                res[i] = (day - i) as i32;
            }
        }

        res
    }

    // 1091. Shortest Path in Binary Matrix.
    // https://leetcode.com/problems/shortest-path-in-binary-matrix/
    // BFS method, input rewriting.
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 || grid[grid.len() - 1][grid[0].len() - 1] == 1 {
            return -1;
        }

        let dirs: [(i32, i32); 8] = [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];

        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        queue.push_back((0, 0));

        let mut res = 0;
        while !queue.is_empty() {
            res += 1;
            let l = queue.len();
            for _ in 0..l {
                let cell = queue.pop_front().unwrap();
                if cell.0 < 0 || cell.1 < 0 {
                    continue;
                }

                let r = cell.0 as usize;
                let c = cell.1 as usize;

                if r == grid.len() || c == grid[0].len() || grid[r][c] != 0 {
                    continue;
                }

                if r == grid.len() - 1 && c == grid[0].len() - 1 {
                    return res;
                }

                grid[r][c] = 2;

                for d in &dirs {
                    queue.push_back((cell.0 + d.0, cell.1 + d.1));
                }
            }
        }

        -1
    }

    // 1015. Smallest Integer Divisible by K.
    // https://leetcode.com/problems/smallest-integer-divisible-by-k/
    // Time complexity: O(K).
    // Space complexity: O(1).
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut rem = 0;
        for n_len in 1..=k {
            rem = (rem * 10 + 1) % k;
            if rem == 0 {
                return n_len;
            }
        }

        -1
    }

    // 1026. Maximum Difference Between Node and Ancestor.
    // https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut min: i32, mut max: i32) -> i32 {
            if let Some(inner_node) = node {
                let val = inner_node.borrow().val;
                min = min.min(val);
                max = max.max(val);

                let left = dfs(&inner_node.borrow().left, min, max);
                let right = dfs(&inner_node.borrow().right, min, max);

                left.max(right)
            } else {
                max - min
            }
        }

        let val = root.as_ref().unwrap().borrow().val;
        dfs(&root, val, val)
    }

    // 2170. Minimum Operations to Make the Array Alternating
    // https://leetcode.com/problems/minimum-operations-to-make-the-array-alternating/
    // Time complexity: O(N).
    // Space complexity: O(Domain(N)).
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut odd_count = vec![0; 100001];
        let mut even_count = vec![0; 100001];
        for i in 0..nums.len() {
            if i % 2 == 0 {
                even_count[nums[i] as usize] += 1;
            } else {
                odd_count[nums[i] as usize] += 1;
            }
        }

        let mut odd_most_freq = 0;
        let mut even_most_freq = 0;
        for i in 1..100001 {
            if odd_count[i] > odd_count[odd_most_freq] {
                odd_most_freq = i;
            }
            if even_count[i] > even_count[even_most_freq] {
                even_most_freq = i;
            }
        }

        nums.len() as i32
            - if odd_most_freq != even_most_freq {
                odd_count[odd_most_freq] + even_count[even_most_freq]
            } else {
                let mut odd_2nd_most_freq = 0;
                let mut even_2nd_most_freq = 0;
                for i in 1..100001 {
                    if i != odd_most_freq && odd_count[i] > odd_count[odd_2nd_most_freq] {
                        odd_2nd_most_freq = i;
                    }
                    if i != even_most_freq && even_count[i] > even_count[even_2nd_most_freq] {
                        even_2nd_most_freq = i;
                    }
                }

                if odd_count[odd_most_freq] + even_count[even_2nd_most_freq]
                    > odd_count[odd_2nd_most_freq] + even_count[even_most_freq]
                {
                    odd_count[odd_most_freq] + even_count[even_2nd_most_freq]
                } else {
                    odd_count[odd_2nd_most_freq] + even_count[even_most_freq]
                }
            }
    }

    // 539. Minimum Time Difference.
    // https://leetcode.com/problems/minimum-time-difference/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes = vec![false; 24 * 60];
        let mut min = 24 * 60;
        let mut max = 0;
        for i in 0..time_points.len() {
            let bytes = time_points[i].as_bytes();
            let hh = &bytes[0..2];
            let mm = &bytes[3..5];
            let m = ((hh[0] - b'0') * 10 + hh[1] - b'0') as usize * 60
                + (mm[0] - b'0') as usize * 10
                + (mm[1] - b'0') as usize;
            if minutes[m] {
                return 0;
            }
            min = min.min(m);
            max = max.max(m);
            minutes[m] = true;
        }

        let mut min_diff = min + 24 * 60 - max;
        let mut prev = min;
        for curr in (min + 1)..=max {
            if minutes[curr] {
                min_diff = min_diff.min(curr - prev);
                if min_diff == 1 {
                    break;
                }
                prev = curr;
            }
        }

        min_diff as _
    }

    // 2182. Construct String With Repeat Limit.
    // https://leetcode.com/problems/construct-string-with-repeat-limit/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let limit = repeat_limit as usize;
        let mut count = vec![0; 26];
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            count[(bytes[i] - b'a') as usize] += 1;
        }

        let mut curr: i32 = 25;
        let mut next = curr - 1;
        let mut res = Vec::with_capacity(bytes.len());
        while curr >= 0 {
            let repeat = limit.min(count[curr as usize]);
            count[curr as usize] -= repeat;
            for _ in 0..repeat {
                res.push((curr as u8 + b'a') as char);
            }

            if count[curr as usize] > 0 {
                if curr <= next {
                    next = curr - 1;
                }
                while next >= 0 && count[next as usize] == 0 {
                    next -= 1;
                }
                if next >= 0 {
                    count[next as usize] -= 1;
                    res.push((next as u8 + b'a') as char);
                    continue;
                } else {
                    break;
                }
            };

            curr -= 1;
        }

        res.into_iter().collect()
    }
}
