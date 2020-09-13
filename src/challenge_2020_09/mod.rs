mod moving_average;

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{Ordering, min, max};

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

    // 165. Compare Version Numbers.
    // https://leetcode.com/problems/compare-version-numbers/
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut p1: Vec<u32> = version1.split('.').map(|x| u32::from_str_radix(x, 10).unwrap()).collect();
        let mut p2: Vec<u32> = version2.split('.').map(|x| u32::from_str_radix(x, 10).unwrap()).collect();
        match p1.len().cmp(&p2.len()) {
            Ordering::Greater => p2.extend_from_slice(&vec![0; p1.len() - p2.len()]),
            Ordering::Less => p1.extend_from_slice(&vec![0; p2.len() - p1.len()]),
            _ => ()
        }
        for (v1, v2) in p1.into_iter().zip(p2.into_iter()) {
            match v1.cmp(&v2) {
                Ordering::Greater => return 1,
                Ordering::Less => return -1,
                _ => (),
            }
        }
        0
    }

    // 299. Bulls and Cows.
    // https://leetcode.com/problems/bulls-and-cows/
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut s_counter = vec![0; 10];
        let mut g_counter = vec![0; 10];
        let mut bulls = 0;
        for (s, g) in secret.chars().zip(guess.chars()) {
            if s == g {
                bulls += 1;
            } else {
                s_counter[s as usize - 48] += 1;
                g_counter[g as usize - 48] += 1;
            }
        }

        let mut cows = 0;
        for i in 0..=9 {
            cows += min(s_counter[i], g_counter[i]);
        }

        format!("{}A{}B", bulls, cows)
    }

    // 152. Maximum Product Subarray.
    // https://leetcode.com/problems/maximum-product-subarray/
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        let mut max_prod: i32 = nums[0];
        let mut min_prod: i32 = nums[0];
        let mut res = nums[0];
        for n in nums.into_iter().skip(1) {
            let max_prod_temp = max(n, max(max_prod * n, min_prod * n));
            min_prod = min(n, min(max_prod * n, min_prod * n));
            max_prod = max_prod_temp;
            res = max(max_prod, res);
        }
        res
    }

    // 216. Combination Sum III.
    // https://leetcode.com/problems/combination-sum-iii/
    pub fn combination_sum_iii(k: i32, n: i32) -> Vec<Vec<i32>> {
        if k < 1 || k > 9 { return vec![]; }

        fn backtrack(k: i32, n: i32, start: i32) -> Vec<Vec<i32>> {
            // Shortcut.
            if k == 1 {
                return if n < start || n > 9 { vec![] } else { vec![vec![n]] };
            }

            let mut res: Vec<Vec<i32>> = vec![vec![]; 0];
            for i in start..=9 {
                match i.cmp(&n) {
                    Ordering::Equal if k == 1 => {
                        res.push(vec![i]);
                        break;
                    },
                    Ordering::Less if k > 1 && i < 9 => {
                        let sub_res = backtrack(k - 1, n - i, i + 1);
                        if sub_res.is_empty() { continue; }
                        for mut v in sub_res {
                            v.push(i);
                            res.push(v);
                        }
                    },
                    _ => break,
                }
            }
            res
        }

        backtrack(k, n, 1)
    }
}
