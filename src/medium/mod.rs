#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{Ordering, min};

use crate::common::tree_node::TreeNode;

struct Solution {}

impl Solution {
    // 274. H-Index
    // https://leetcode.com/problems/h-index/
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        if citations.is_empty() { return 0; }
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
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(some) = root {
            let left = Solution::remove_leaf_nodes(RefCell::borrow_mut(&some).left.clone(), target);
            let right = Solution::remove_leaf_nodes(RefCell::borrow_mut(&some).right.clone(), target);
            if left.is_none() && right.is_none() && RefCell::borrow(&some).val == target {
                None
            } else {
                RefCell::borrow_mut(&some).left = left.clone();
                RefCell::borrow_mut(&some).right = right.clone();
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
            if node.is_none() { return; }
            if level == levels.len() {
                levels.push(vec![]);
            }
            levels[level].push(RefCell::borrow(node.as_ref().unwrap()).val);
            dfs(RefCell::borrow(node.as_ref().unwrap()).left.clone(), levels, level + 1);
            dfs(RefCell::borrow(node.as_ref().unwrap()).right.clone(), levels, level + 1);
        }

        let mut levels: Vec<Vec<i32>> = Vec::new();
        dfs(root, &mut levels, 0);
        levels
    }

    // 889. Construct Binary Tree from Preorder and Postorder Traversal.
    // https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
    pub fn build_tree(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_bt(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || postorder.is_empty() { return None; }
            let mut val = preorder[0];
            let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            if preorder.len() == 1 { return node; }
            val = preorder[1];
            let i = postorder.iter().position(|x| *x == val).unwrap();
            RefCell::borrow_mut(node.as_ref().unwrap()).left = build_bt(&preorder[1..=1 + i], &postorder[..=i]);
            RefCell::borrow_mut(node.as_ref().unwrap()).right = build_bt(&preorder[i + 2..], &postorder[i + 1..postorder.len() - 1]);
            node
        }

        build_bt(&preorder, &postorder)
    }

    // 105. Construct Binary Tree from Preorder and Inorder Traversal.
    // https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
    pub fn build_tree_ii(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_bt(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || inorder.is_empty() { return None; }
            let val = preorder[0];
            let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            if preorder.len() == 1 { return node; }
            let i = inorder.iter().position(|x| *x == val).unwrap();
            RefCell::borrow_mut(node.as_ref().unwrap()).left = build_bt(&preorder[1..i + 1], &inorder[..i]);
            RefCell::borrow_mut(node.as_ref().unwrap()).right = build_bt(&preorder[i + 1..], &inorder[i + 1..]);
            node
        }

        build_bt(&preorder, &inorder)
    }

    // 39. Combination Sum.
    // https://leetcode.com/problems/combination-sum/
    // Backtracking solution.
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        fn backtrack(candidates: &Vec<i32>, target: i32, start: usize) -> Vec<Vec<i32>> {
            let mut res: Vec<Vec<i32>> = vec![vec![]; 0];
            for i in start..candidates.len() {
                match candidates[i].cmp(&target) {
                    Ordering::Equal => {
                        res.push(vec![candidates[i]]);
                        break;
                    },
                    Ordering::Less => {
                        for mut v in backtrack(candidates, target - candidates[i], i) {
                            v.push(candidates[i]);
                            res.push(v);
                        }
                    },
                    _ => break,
                }
            }
            res
        }

        backtrack(&candidates, target, 0)
    }

    // DP solution.
    // pub fn combination_sum_v2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    //
    // }

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
                    },
                    Ordering::Less if i < candidates.len() - 1 => {
                        // Extra check to eliminate duplicate sets.
                        // We don't need to analyze sets with the same number at the same position,
                        // because we analyzed them at previous iteration.
                        if i > start && candidates[i - 1] == candidates[i] { continue; }

                        for mut v in backtrack(candidates, target - candidates[i], i + 1) {
                            v.push(candidates[i]);
                            res.push(v);
                        }
                    },
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
        if amount == 0 { return 0; }
        let mut dp: Vec<i32> = vec![i32::MAX; (amount + 1) as usize];
        dp[0] = 0;
        for a in 1..=amount {
            for c in &coins {
                if a - c >=0 {
                    let p = dp[(a - c) as usize];
                    dp[a as usize] = min(dp[a as usize], if p == i32::MAX { i32::MAX } else { p + 1 });
                }
            }
        }
        let res = dp[amount as usize];
        if res == i32::MAX { -1 } else { res }
    }

    // 8. String to Integer (atoi).
    // https://leetcode.com/problems/string-to-integer-atoi/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn my_atoi(s: String) -> i32 {
        let chars: Vec<char> = s.chars()
            .skip_while(|x| x.is_ascii_whitespace())
            .collect();
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

        let digits: Vec<i32> = chars.into_iter()
            .skip(skip)
            .skip_while(|&x| x == '0')
            .take_while(|x| x.is_digit(10))
            .map(|x| match x {
                '0' => 0, '1' => 1, '2' => 2, '3' => 3, '4' => 4, '5' => 5, '6' => 6, '7' => 7, '8' => 8, '9' => 9, _=> unreachable!()
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

    // pub fn longest_palindrome(s: String) -> String {
    //     let chars: Vec<char> = s.chars().collect();
    //     let l = chars.len();
    //     let mut max_pal_str: &[char] = &chars[0..1];
    //     for pal_len in 2..l / 2 {
    //         println!("pal_len={}", pal_len);
    //         for i in 0..(l - pal_len) {
    //             let mut start = i;
    //             let mut end = i + pal_len;
    //             println!("start={}, end={}", start, end);
    //             while start < end {
    //                 if chars[start] != chars[end] {
    //                     break;
    //                 }
    //                 start += 1;
    //                 end -= 1;
    //             }
    //             if start >= end {
    //                 max_pal_str = &chars[i..(i + pal_len)];
    //                 break;
    //             }
    //         }
    //     }
    //     max_pal_str.iter().collect()
    // }
}
