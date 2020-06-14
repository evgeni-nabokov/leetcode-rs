mod pick_index;
mod randomized_set;
mod randomized_set_v2;

#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

use crate::common::tree_node::TreeNode;

struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(some) = root {
            let left = RefCell::borrow_mut(&some).left.clone();
            let right = RefCell::borrow_mut(&some).right.clone();
            if left.is_none() && right.is_some() {
                RefCell::borrow_mut(&some).left = right;
                RefCell::borrow_mut(&some).right = None;
                Solution::invert_tree(RefCell::borrow_mut(&some).left.clone());
            } else if left.is_some() && right.is_none() {
                RefCell::borrow_mut(&some).right = left;
                RefCell::borrow_mut(&some).left = None;
                Solution::invert_tree(RefCell::borrow_mut(&some).right.clone());
            } else if left.is_some() && right.is_some() {
                let node = RefCell::borrow(&some);
                node.left.as_ref().unwrap().swap(node.right.as_ref().unwrap());
                Solution::invert_tree(node.left.clone());
                Solution::invert_tree(node.right.clone());
            }
            Some(some)
        } else {
            None
        }
    }

    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_unstable_by_key(|a| a[0] - a[1]);
        let n = costs.len() / 2;
        costs.iter().take(n).map(|x| x[0]).sum::<i32>() + costs.iter().skip(n).map(|x| x[1]).sum::<i32>()
    }

    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() { return; }
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|x, y| match x[0].cmp(&y[0]) {
            Ordering::Equal => x[1].cmp(&y[1]),
            o => o,
        });
        let mut res: Vec<Vec<i32>> = vec![vec![]; people.len()];
        for p in people.iter() {
            let mut cnt = p[1];
            let mut i = 0usize;
            loop {
                if res[i].is_empty() || res[i][0] >= p[0] {
                    if cnt == 0 {
                        break;
                    }
                    cnt -= 1;
                }
                i += 1
            }
            res[i] = p.clone();
        }
        res
    }

    pub fn reconstruct_queue_v2(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|x, y| match y[0].cmp(&x[0]) {
            Ordering::Equal => x[1].cmp(&y[1]),
            o => o,
        });
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(people.len());
        for p in people.iter() {
            res.insert(p[1] as usize, p.clone());
        }
        res
    }

    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        if amount == 0 { return 1; }
        if coins.is_empty() { return 0; }
        let mut table = vec![vec![0; amount as usize]; coins.len()];
        for row in 0..coins.len() {
            for col in 0..amount as usize {
                let amt = col as i32 + 1;
                let cn = coins[row];
                table[row][col] = match amt - cn {
                    0 => 1,
                    d if d > 0 => table[row][d as usize - 1],
                    _ => 0
                } + if row == 0 { 0 } else { table[row - 1][col] };
            }
        }
        table[coins.len() - 1][amount as usize - 1]
    }

    pub fn is_power_of_two(mut n: i32) -> bool {
        if n <= 0 { return false; }
        loop {
            if n == 1 {
                return true;
            }
            if n % 2 == 1 {
                return false;
            }
            n /= 2;
        }
    }

    pub fn is_power_of_two_v2(n: i32) -> bool {
        n > 0 && n == n & -n
    }

    pub fn is_power_of_two_v3(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }

    pub fn is_subsequence(s: String, t: String) -> bool {
        if !s.is_empty() && t.is_empty() { return false; }
        if s.is_empty() { return true; }
        let mut i = 0;
        let s_chars: Vec<char> = s.chars().collect();
        for c in t.chars() {
            if s_chars[i] == c {
                i += 1;
                if i == s.len() {
                    return true;
                }
            }
        }
        i == s.len()
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        (match nums.binary_search(&target) {
            Ok(i) => i,
            Err(i) => i,
        }) as i32
    }

    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() < 2 { return; }
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut i = 0;
        while i <= right {
            let n = nums[i];
            match n {
                0 => {
                    nums.swap(i, left);
                    left += 1;
                    i += 1;
                },
                2 => {
                    nums.swap(i, right);
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                },
                _ => i += 1,
            }
        }
    }

    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 { return nums; }
        let mut div_count: Vec<usize> = vec![1; nums.len()];
        let mut prev: Vec<isize> = vec![-1; nums.len()];
        nums.sort_unstable();
        let mut max_idx = 0;
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && div_count[i] < div_count[j] + 1 {
                    prev[i] = j as isize;
                    div_count[i] = div_count[j] + 1;
                }
            }
            if div_count[max_idx] < div_count[i] {
                max_idx = i;
            }
        }
        let mut res: Vec<i32> = Vec::new();
        let mut i  = max_idx as isize;
        while i >= 0 {
            res.push(nums[i as usize]);
            i = prev[i as usize];
        }
        res
    }

}