use std::collections::{HashSet, HashMap};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

use crate::common::tree_node::TreeNode;

mod logger_v1;
mod logger_v2;
mod hash_set;
mod trie_node;
mod word_dictionary;

#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 520. Detect Capital.
    // https://leetcode.com/problems/detect-capital/
    pub fn detect_capital_use(word: String) -> bool {
        let mut is_upper_allowed = true;
        let mut is_lower_allowed = true;
        for (i, c) in word.chars().enumerate() {
            match (c.is_ascii_uppercase(), is_upper_allowed, is_lower_allowed) {
                (false, _, true) => { is_upper_allowed = false; },
                (true, false, _) | (false, _, false) => { return false; },
                (true, _, _ ) if i == 1 => { is_lower_allowed = false; }
                _ => (),
            }
        }
        true
    }

    // 125. Valid Palindrome.
    // https://leetcode.com/problems/valid-palindrome/
    pub fn is_palindrome(s: String) -> bool {
        if s.len() < 2 { return true; }
        let mut left = 0;
        let mut right = s.len() - 1;
        let chars: Vec<_> = s.chars().collect();
        loop {
            while left < right && !chars[left].is_alphanumeric() {
                left += 1;
            }
            while left < right && !chars[right].is_alphanumeric() {
                right -= 1;
            }
            if !chars[left].eq_ignore_ascii_case(&chars[right]) {
                return false;
            }
            if left >= right {
                return true;
            }
            left += 1;
            right -= 1;
        }
        true
    }

    // 342. Power of Four.
    // https://leetcode.com/problems/power-of-four/
    pub fn is_power_of_four(mut num: i32) -> bool {
        if num == 1 { return true; }
        if num < 4 || num & (num - 1) != 0 { return false; }
        let mut count = 0;
        while num > 1 {
            num >>= 1;
            count += 1;
        }
        count % 2 == 0
    }

    pub fn is_power_of_four_v2(num: i32) -> bool {
        let is_power_of_two = num > 0 && num & (-num) == num;
        is_power_of_two && (num & 0b1010101010101010101010101010101 != 0)
    }

    pub fn is_power_of_four_v3(num: i32) -> bool {
        let is_power_of_two = num > 0 && num & (-num) == num;
        is_power_of_two && num % 3 == 1
    }

    // 442. Find All Duplicates in an Array.
    // https://leetcode.com/problems/find-all-duplicates-in-an-array/
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len() / 2);
        for n in nums {
            if set.contains(&n) {
                set.remove(&n);
                res.push(n);
            } else {
                set.insert(n);
            }
        }
        res
    }

    pub fn find_duplicates_v2(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            let n = nums[i].abs();
            let j = n as usize - 1;
            if nums[j] > 0 {
                nums[j] = -nums[j];
            } else {
                res.push(n);
            }
        }
        res
    }

    // 987. Vertical Order Traversal of a Binary Tree.
    // https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32, map: &mut HashMap<i32, Vec<(i32, i32)>>) {
            if let Some(some) = node {
                map.entry(x).or_insert(Vec::new()).push((y, RefCell::borrow(&some).val));
                dfs(&RefCell::borrow(&some).left, x - 1, y - 1, map);
                dfs(&RefCell::borrow(&some).right, x + 1, y - 1, map);
            }
        }

        let mut map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        dfs(&root, 0, 0, &mut map);
        let mut vec: Vec<_> = map.into_iter().map(|(x, a)| (x, a)).collect();
        vec.sort_unstable_by_key(|a| a.0);
        vec
            .into_iter()
            .map(|(x, mut yv)| {
                yv.sort_unstable_by(|(y1, v1), (y2, v2)|
                    match y2.cmp(y1) {
                        Ordering::Equal => v1.cmp(v2),
                        a => a,
                    }
                );
                yv.into_iter().map(|(y, v)| v).collect()
            })
            .collect()
    }

    pub fn vertical_traversal_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32, list: &mut Vec<(i32, i32, i32)>) {
            if let Some(some) = node {
                list.push((x, y, RefCell::borrow(&some).val));
                dfs(&RefCell::borrow(&some).left, x - 1, y - 1, list);
                dfs(&RefCell::borrow(&some).right, x + 1, y - 1, list);
            }
        }

        let mut list = Vec::new();
        dfs(&root, 0, 0, &mut list);
        list.sort_unstable_by(|(x1, y1, v1), (x2, y2, v2)|
              match x1.cmp(x2) {
                  Ordering::Equal => match y2.cmp(y1) {
                      Ordering::Equal => v1.cmp(v2),
                      a => a,
                  },
                  b => b,
              }
        );
        let mut slice_start = 0;
        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in 1..=list.len() {
            if i == list.len() || list[i].0 != list[i - 1].0 {
                res.push(list[slice_start..i].iter().map(|(x, y, v)| *v).collect());
                slice_start = i;
            }
        }
        res
    }

    // 270. Closest Binary Search Tree Value.
    // https://leetcode.com/problems/closest-binary-search-tree-value/
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        fn get_closest_val(val_1: i32, val_2: i32, target: f64) -> i32 {
            if (val_1 as f64 - target).abs() < (val_2 as f64 - target).abs() {
                val_1
            } else {
                val_2
            }
        }

        fn dfs(node: &Rc<RefCell<TreeNode>>, target: f64) -> i32 {
            let n = RefCell::borrow(node);
            if ((n.val as f64) - target).abs() > 0.5 {
                if (n.val as f64) > target && n.left.is_some() {
                    get_closest_val(dfs(n.left.as_ref().unwrap(), target), n.val, target)
                } else if (n.val as f64) < target && n.right.is_some() {
                    get_closest_val(dfs(n.right.as_ref().unwrap(), target), n.val, target)
                } else {
                    n.val
                }
            } else {
                n.val
            }
        }

        dfs(root.as_ref().unwrap(), target)
    }

    pub fn closest_value_v2(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        let mut closest = RefCell::borrow(root.as_ref().unwrap()).val;
        let mut diff = (closest as f64 - target).abs();
        let mut node = root;
        while let Some(some) = node {
            let n = RefCell::borrow(&some);
            if (n.val as f64 - target).abs() < diff {
                closest = n.val;
                diff = (closest as f64 - target).abs();
            }
            if target < n.val as f64 {
                node = n.left.clone();
            } else if target > n.val as f64 {
                node = n.right.clone();
            } else {
                break;
            }
        }
        closest
    }
}