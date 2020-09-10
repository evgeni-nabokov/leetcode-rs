mod moving_average;

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;
use std::cell::RefCell;

use crate::common::tree_node::TreeNode;

struct Solution;

impl Solution {
    // 949. Largest Time for Given Digits.
    // https://leetcode.com/problems/largest-time-for-given-digits/
    pub fn largest_time_from_digits(mut a: Vec<i32>) -> String {
        a.sort_unstable();

        fn pop_less_than(mut val: i32, sorted_vec: &mut Vec<i32>) -> Option<i32> {
            loop {
                val -= 1;
                if let Ok(i) = sorted_vec.binary_search(&val) {
                    let v = sorted_vec[i];
                    sorted_vec.remove(i);
                    return Some(v);
                } else if val == 0 {
                    return None;
                }
            }
        }

        let mut res: Vec<i32> = Vec::with_capacity(4);

        // 1st digit of hours: 0..=1 or 0..=2 depends on how many numbers greater than 5 we have.
        if let Some(c) = pop_less_than(if a[2] > 5 { 2 } else { 3 }, &mut a) {
            res.push(c);
        } else {
            return "".to_string();
        }

        // 2nd digit of hours: 0..=9 or 0..=3 depends of the 1st digit.
        if let Some(c) = pop_less_than(if res[0] < 2 { 10 } else { 4 }, &mut a) {
            res.push(c);
        } else {
            return "".to_string();
        }

        // 1st digit of minutes: 0..=5
        if let Some(c) = pop_less_than(6, &mut a) {
            res.push(c);
        } else {
            return "".to_string();
        }

        // 2st digit of minutes: 0..=9
        res.push(a.pop().unwrap());

        format!("{}{}:{}{}", res[0], res[1], res[2], res[3])
    }

    // 220. Contains Duplicate III.
    // https://leetcode.com/problems/contains-duplicate-iii/
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as usize;
        let t = t as i64;
        let mut set: BTreeSet<i64> = BTreeSet::new();
        for i in 0..nums.len() {
            let n = nums[i] as i64;

            if let Some(&s) = set.range(n..).next() {
                if s <= n + t { return true; }
            }

            if let Some(&s) = set.range(..=n).next_back() {
                if s >= n - t { return true; }
            }

            set.insert(n);
            if set.len() > k {
                set.remove(&(nums[i - k] as i64));
            }
        }
        false
    }

    // 459. Repeated Substring Pattern.
    // https://leetcode.com/problems/repeated-substring-pattern/
    pub fn repeated_substring_pattern(s: String) -> bool {
        if s.len() < 2 { return false; }
        let mut together = format!("{}{}", s, s);
        together.remove(0);
        together.remove(together.len() - 1);
        together.contains(&s)
    }

    // 290. Word Pattern.
    // https://leetcode.com/problems/word-pattern/
    pub fn word_pattern(pattern: String, str: String) -> bool {
        if pattern.is_empty() || str.is_empty() { return false; }

        let p_chars: Vec<char> = pattern.chars().collect();
        let s_words: Vec<&str> = str.split_ascii_whitespace().collect();
        if p_chars.len() != s_words.len() { return false; }

        let mut map: Vec<Option<&str>> = vec![None; 26];
        let mut set: HashSet<&str> = HashSet::new();
        for i in 0..p_chars.len() {
            let ch_i = p_chars[i] as usize - 97;
            if let Some(w) = map[ch_i] {
                if s_words[i] != w {
                    return false;
                }
            } else {
                if set.contains(s_words[i]) {
                    return false;
                }
                map[ch_i] = Some(s_words[i]);
                set.insert(s_words[i]);
            }
        }
        true
    }

    // 1022. Sum of Root To Leaf Binary Numbers.
    // https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut n: u32) -> u32 {
            if let Some(node_inner) = node {
                let node_borrowed = node_inner.borrow();
                n = (n << 1) ^ (node_borrowed.val as u32);
                if node_borrowed.left.is_none() && node_borrowed.right.is_none() {
                    n
                } else {
                    dfs(&node_borrowed.left, n) + dfs(&node_borrowed.right, n)
                }
            } else {
                0
            }
        }

        dfs(&root, 0) as i32
    }


}
