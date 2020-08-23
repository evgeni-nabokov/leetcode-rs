use std::collections::{HashSet, HashMap, VecDeque};
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
            .map(|(_, mut yv)| {
                yv.sort_unstable_by(|(y1, v1), (y2, v2)|
                    match y2.cmp(y1) {
                        Ordering::Equal => v1.cmp(v2),
                        a => a,
                    }
                );
                yv.into_iter().map(|(_, v)| v).collect()
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
                res.push(list[slice_start..i].iter().map(|(_, _, v)| *v).collect());
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

    // 437. Path Sum III.
    // https://leetcode.com/problems/path-sum-iii/
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, k: i32, mut sum: i32, sums: &mut HashMap<i32, i32>) -> i32 {
            if let Some(some) = node {
                let n = RefCell::borrow(&some);
                sum += n.val;
                let key = sum - k;
                let mut cnt = *sums.get(&key).unwrap_or(&0);
                *sums.entry(sum).or_insert(0) += 1;
                cnt += dfs(n.left.clone(), k, sum, sums) + dfs(n.right.clone(), k, sum, sums);
                *sums.get_mut(&sum).unwrap() -= 1;
                cnt
            } else {
                0
            }
        }

        let mut sums: HashMap<i32, i32> = HashMap::new();
        sums.insert(0, 1);
        dfs(root, k, 0, &mut sums)
    }

    // 994. Rotting Oranges.
    // https://leetcode.com/problems/rotting-oranges/
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() { return -1; }
        let mut fresh_cnt = 0usize;
        let mut rotten_oranges: VecDeque<(isize, isize)> = VecDeque::new();
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                match grid[r][c] {
                    1 => fresh_cnt += 1,
                    2 => rotten_oranges.push_back((r as isize, c as isize)),
                    _ => (),
                }
            }
        }
        let mut step = 0;
        loop {
            for _ in 0..rotten_oranges.len() {
                let (r, c) = rotten_oranges.pop_front().unwrap();
                for (y, x) in vec![(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                    let are_xy_valid = y >= 0 && y < grid.len() as isize && x >= 0 && x < grid[y as usize].len() as isize;
                    if are_xy_valid && grid[y as usize][x as usize] == 1 {
                        grid[y as usize][x as usize] = 2;
                        fresh_cnt -= 1;
                        rotten_oranges.push_back((y, x));
                    }
                }
            }
            if rotten_oranges.is_empty() { break; }
            step += 1;
        }
        if fresh_cnt == 0 { step } else { -1 }
    }

    // 171. Excel Sheet Column Number
    // https://leetcode.com/problems/excel-sheet-column-number/
    pub fn title_to_number(s: String) -> i32 {
        s.chars().fold(0, |acc, c| acc * 26 + (c as i32 - 64))
    }

    // 119. Pascal's Triangle II.
    // https://leetcode.com/problems/pascals-triangle-ii/
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row: Vec<i32> = Vec::with_capacity(row_index as usize + 1);
        row.push(1);
        for i in 0..row_index {
            row.push((row[i as usize] as u64 * (row_index - i) as u64 / (i + 1) as u64) as i32);
        }
        row
    }

    // 409. Longest Palindrome.
    // https://leetcode.com/problems/longest-palindrome/
    pub fn longest_palindrome(s: String) -> i32 {
        let mut char_counter = vec![0; 52];
        for c in s.chars() {
            let mut i = c as usize;
            i = if i > 90 { i - 71 } else { i - 65 };
            char_counter[i] += 1;
        }
        let mut counter = 0;
        for cnt in char_counter {
            counter += cnt / 2 * 2;
            if counter % 2 == 0 && cnt % 2 == 1 {
                counter += 1;
            }
        }
        counter
    }

    // 824. Goat Latin.
    // https://leetcode.com/problems/goat-latin/
    pub fn to_goat_latin(s: String) -> String {
        let mut transformed_words: Vec<String> = Vec::new();
        for (i, w) in s.split_ascii_whitespace().enumerate() {
            let mut word_chars: VecDeque<_> = w.chars().collect();
            match word_chars[0] {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => word_chars.extend(&['m', 'a']),
                _ => {
                    let first = word_chars.pop_front().unwrap();
                    word_chars.extend(&[first, 'm', 'a'])
                }
            }
            word_chars.extend(vec!['a'; i + 1]);
            transformed_words.push(word_chars.iter().collect());
        }
        transformed_words.join(" ")
    }

    // 905. Sort Array By Parity.
    // https://leetcode.com/problems/sort-array-by-parity/
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        a.sort_unstable_by(|x, y| match (x % 2 == 0, y % 2 == 0) {
            (true, true) | (false, false) => x.cmp(y),
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
        });
        a
    }

    pub fn sort_array_by_parity_v2(mut a: Vec<i32>) -> Vec<i32> {
        a.sort_by_key(|k| k % 2);
        a
    }
}