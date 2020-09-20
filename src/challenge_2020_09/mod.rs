mod moving_average;

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{Ordering, min, max};

use lazy_static::lazy_static;

use crate::common::tree_node::TreeNode;

lazy_static! {
    static ref SEQ_DIG_NUMBERS: Vec<i32> = {
        let mut res: Vec<i32> = Vec::new();
        let mut digits: Vec<u8> = Vec::with_capacity(9);
        for i in 2..=9 {
            for j in 1..=(10 - i) {
                let mut d = j;
                for _ in 1..=i {
                    digits.push(d);
                    d += 1;
                }
                res.push(digits.iter().fold(0, |p, &x| p * 10 + x as i32));
                digits.clear();
            }
        }
        res
    };
}

const POWERS_OF_10: [u32; 9] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000];
const DIGITS: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

fn get_num_len(num: u32) -> usize {
    match POWERS_OF_10.binary_search(&num) {
        Ok(i) => i + 1,
        Err(i) => i,
    }
}

fn concat_digits(digits: &[u32]) -> i32 {
    digits.iter().fold(0, |p, &d| p * 10 + d as i32)
}

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

    // 57. Insert Interval.
    // https://leetcode.com/problems/insert-interval/
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        fn search_containing_interval(intervals: &Vec<Vec<i32>>, p: i32) -> Result<usize, usize> {
            match intervals.binary_search_by_key(&p, |x| x[0]) {
                Ok(x) => Ok(x),
                Err(x) if x > 0 && intervals[x - 1][1] >= p => Ok(x - 1),
                Err(x) => Err(x)
            }
        }

        let mut res: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        let start = search_containing_interval(&intervals, new_interval[0]);
        let end = search_containing_interval(&intervals, new_interval[1]);
        match (start, end) {
            (Ok(s), Ok(e)) => {
                res.append(intervals[..s].to_vec().as_mut());
                res.push(vec![intervals[s][0], intervals[e][1]]);
                res.append(intervals[e + 1..].to_vec().as_mut());
            },
            (Err(s), Err(e)) => {
                res.append(intervals[..s].to_vec().as_mut());
                res.push(new_interval);
                res.append(intervals[e..].to_vec().as_mut());
            },
            (Ok(s), Err(e)) => {
                res.append(intervals[..s].to_vec().as_mut());
                res.push(vec![intervals[s][0], new_interval[1]]);
                res.append(intervals[e..].to_vec().as_mut());
            },
            (Err(s), Ok(e)) => {
                res.append(intervals[..s].to_vec().as_mut());
                res.push(vec![new_interval[0], intervals[e][1]]);
                res.append(intervals[e + 1..].to_vec().as_mut());
            },
        }
        res
    }

    // 198. House Robber.
    // https://leetcode.com/problems/house-robber/
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 0; }
        if nums.len() == 1 { return nums[0]; }

        nums[1] = max(nums[0], nums[1]);
        for i in 2..nums.len() {
            nums[i] = max(nums[i - 2] + nums[i], nums[i - 1]);
        }

        nums.pop().unwrap()
    }

    // 1041. Robot Bounded In Circle.
    // https://leetcode.com/problems/robot-bounded-in-circle/
    pub fn is_robot_bounded(instructions: String) -> bool {
        #[derive(PartialEq, Eq, Clone, Debug)]
        struct Point { x: i32, y: i32 };

        #[derive(PartialEq, Eq, Clone, Debug)]
        enum Direction { North, West, South, East }

        let init_point = Point { x: 0, y: 0 };
        let init_dir = Direction::North;

        let mut curr_point = Point { x: 0, y: 0 };
        let mut curr_dir = Direction::North;

        let commands: Vec<char> = instructions.chars().collect();
            for c in &commands {
                match c {
                    'L' => curr_dir = match curr_dir {
                        Direction::North => Direction::West,
                        Direction::West => Direction::South,
                        Direction::South => Direction::East,
                        Direction::East => Direction::North,
                    },
                    'R' => curr_dir = match curr_dir {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    },
                    'G' => match curr_dir {
                        Direction::North => curr_point.y += 1,
                        Direction::East => curr_point.x += 1,
                        Direction::South => curr_point.y -= 1,
                        Direction::West => curr_point.x -= 1,
                    },
                    _ => (),
                }
        }
        curr_dir != init_dir || curr_point == init_point
    }

    // 121. Best Time to Buy and Sell Stock.
    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 { return 0; }
        let mut min_price = prices[0];
        let mut max_profit = 0;
        for i in 1..prices.len() {
            min_price = min(min_price, prices[i]);
            max_profit = max(max_profit, prices[i] - min_price);
        }
        max_profit
    }

    // 1291. Sequential Digits.
    // https://leetcode.com/problems/sequential-digits/
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let start = match SEQ_DIG_NUMBERS.binary_search(&low) {
            Ok(i) => i,
            Err(i) => i
        };
        let end = match SEQ_DIG_NUMBERS.binary_search(&high) {
            Ok(i) => i + 1,
            Err(i) => i
        };

        let mut res: Vec<i32> = Vec::new();
        for i in start..end {
            res.push(SEQ_DIG_NUMBERS[i]);
        }
        res
    }

    // Sliding window solution.
    pub fn sequential_digits_v2(low: i32, high: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        for l in get_num_len(low as u32)..=get_num_len(high as u32) {
            for i in 0..=9 - l {
                let num = concat_digits(&DIGITS[i..i + l]);
                if num >= low && num <= high {
                    res.push(num);
                }
            }
        }
        res
    }
}
