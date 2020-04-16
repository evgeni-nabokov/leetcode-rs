#[cfg(test)]
mod tests;
mod list_node;
mod tree_node;
mod min_stack;

use std::cmp::{max, min};
use std::iter::Rev;
use std::str::Chars;
use std::collections::{HashSet, HashMap, BinaryHeap};

use crate::challenge::list_node::ListNode;
use crate::challenge::tree_node::TreeNode;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/single-number/
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.push(0); // A stub for one more iteration.
        let mut prev: &i32 = &nums[0];
        let mut cnt: u32 = 1;
        for n in nums.iter().skip(1) {
            if *n == *prev {
                cnt += 1;
            } else {
                if cnt == 1 {
                    return *prev;
                } else {
                    cnt = 1;
                    prev = n;
                }
            }
        }
        0
    }

    pub fn single_number_v2(nums: Vec<i32>) -> i32 {
        let mut a: i32 = 0;
        for n in nums.iter() {
            a ^= *n;
        }
        a
    }

    // https://leetcode.com/problems/happy-number/
    pub fn is_happy(n: i32) -> bool {
        if n == 0 { return false; }
        if n == 1 { return true; }
        let mut nn = n.clone();
        let mut sum_set: HashSet<i32> = HashSet::with_capacity(20);
        sum_set.insert(nn);
        let mut sum = 0;
        loop {
            // Splitting into separate digits, square them and sum.
            while nn > 0 {
                let d = nn % 10;
                sum += d * d;
                nn = nn / 10;
            }
            if sum == 1 {
                return true;
            } else if sum_set.contains(&sum) {
                return false;
            }
            sum_set.insert(sum);
            nn = sum;
            sum = 0;
        }
    }

    // https://leetcode.com/problems/maximum-subarray/
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        let mut sum: i32 = nums[0];
        let mut max_sum: i32 = sum;
        for &n in nums.iter().skip(1) {
            match n {
                n if n >= 0 && sum < 0 => sum = n,
                n if n >= 0 && sum >= 0 => sum += n,
                n if n < 0 && sum < 0 && sum < n => sum = n,
                n if n < 0 && sum > 0 => {
                    if sum > max_sum {
                        max_sum = sum;
                    }
                    sum += n;
                }
                _ => ()
            }
        }
        if sum > max_sum {
            max_sum = sum;
        }
        max_sum
    }

    pub fn max_sub_array_v2(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        let mut sum: i32 = nums[0];
        let mut max_sum: i32 = sum;
        for &n in nums.iter().skip(1) {
            sum = max(n, sum + n);
            max_sum = max(sum, max_sum);
        }
        max_sum
    }

    // https://leetcode.com/problems/move-zeroes/
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() < 2 { return; }
        let mut i: usize = 0;
        let mut cnt: usize = 0;
        let len = nums.len();
        loop {
            if i == len {
                break;
            } else if nums[i] == 0 {
                cnt += 1;
            } else if cnt > 0 {
                nums[i - cnt] = nums[i];
                nums[i] = 0;
            }
            i += 1;
        }
    }

    pub fn move_zeroes_v2(nums: &mut Vec<i32>) {
        if nums.len() < 2 { return; }
        let mut main_index: usize = 0;
        let mut non_zero_index: usize = 0;
        let len = nums.len();
        loop {
            if main_index == len {
                break;
            } else if nums[main_index] != 0 {
                nums.swap(non_zero_index, main_index);
                non_zero_index += 1;
            }
            main_index += 1;
        }
    }

    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 { return 0; }
        let mut in_tran = false;
        let mut i: usize = 0;
        let last_i = prices.len() - 1;
        let mut buy_price = 0;
        let mut profit = 0;
        loop {
            if i == last_i {
                if in_tran {
                    // Final selling.
                    profit += prices[i] - buy_price;
                }
                break;
            }
            if in_tran {
                if prices[i] < prices[i + 1] {
                    // Do nothing.
                } else {
                    // Selling.
                    profit += prices[i] - buy_price;
                    in_tran = false;
                }
            } else {
                if prices[i] < prices[i + 1] {
                    // Buying.
                    buy_price = prices[i];
                    in_tran = true;
                } else {
                    // Do nothing.
                }
            }
            i += 1;
        }
        profit
    }

    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 { return 0; }
        let mut i: usize = 1;
        let len = prices.len();
        let mut profit = 0;
        while i < len {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
            i += 1;
        }
        profit
    }

    // https://leetcode.com/problems/group-anagrams/
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut cntr: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs.iter() {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key: String = chars.into_iter().collect();
            cntr.entry(key).or_insert(Vec::<String>::new()).push(s.clone());
        }
        let mut res: Vec<Vec<String>> = Vec::new();
        for (_, anagrams) in cntr.into_iter() {
            res.push(anagrams);
        }
        for anagrams in res.iter_mut() {
            anagrams.sort_unstable();
        }
        res.sort_unstable_by(|x, y| y.len().cmp(&x.len()));
        res
    }

    pub fn group_anagrams_v2(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut cntr: HashMap<[i32; 26], Vec<String>> = HashMap::new();
        for s in strs.into_iter() {
            let mut key = [0; 26];
            for c in s.chars() {
                key[c as usize - 'a' as usize] += 1;
            }
            cntr.entry(key).or_insert(Vec::<String>::new()).push(s.clone());
        }
        let mut res: Vec<Vec<String>> = cntr.into_iter().map(|(_, s)| s).collect();
        for anagrams in res.iter_mut() {
            anagrams.sort_unstable();
        }
        res.sort_unstable_by(|x, y| y.len().cmp(&x.len()));
        res
    }

    pub fn count_elements(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let mut prev_x = arr[0];
        let mut cnt = 0;
        let mut rep_cnt = 1;
        for &x in arr.iter().skip(1) {
            if x == prev_x {
                rep_cnt += 1;
            } else {
                if prev_x + 1 == x {
                    cnt += rep_cnt;
                }
                rep_cnt = 1;
            }
            prev_x = x;
        }
        cnt
    }

    // https://leetcode.com/problems/middle-of-the-linked-list/
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }
        let mut cur = &head;
        let mut res = &head;
        let mut step = 1;
        let factor = 2;
        while let Some(node) = cur {
            cur = &node.next;
            if step % factor == 0 {
                res = &res.as_ref().unwrap().next;
            }
            step += 1;
        }
        res.clone()
    }

    // https://leetcode.com/problems/backspace-string-compare/
    pub fn backspace_compare(s: String, t: String) -> bool {
        if s.len() == 0 && t.len() == 0 { return true; }
        let mut s_iter = s.chars().rev();
        let mut t_iter = t.chars().rev();
        loop {
            let s_char = apply_backspaces(&mut s_iter);
            let t_char = apply_backspaces(&mut t_iter);
            if s_char.is_none() && t_char.is_none() { return true; }
            if s_char != t_char { return false; }
        }
    }

     // https://leetcode.com/problems/last-stone-weight/
     pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        if stones.is_empty() { return 0; }
        if stones.len() == 1 { return stones[0]; }
        let mut heap= BinaryHeap::with_capacity(30);
        for s in stones.iter() {
            heap.push(*s);
        }
        while heap.len() > 1 {
            let x = heap.pop().unwrap();
            let y = heap.pop().unwrap();
            if x > y {
                heap.push(x - y);
            }
        }
        heap.pop().unwrap_or(0)
    }

    // https://leetcode.com/problems/contiguous-array/
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 { return 0; }
        let mut max_len = 0;
        let mut sum = 0;
        let mut sums = HashMap::<i32, i32>::new();
        sums.insert(0, -1);
        for (i, &n) in nums.iter().enumerate() {
            if n == 0 { sum -= 1; }
            else { sum += 1 }
            if sums.contains_key(&sum) {
                max_len = max(max_len, i as i32 - sums.get(&sum).unwrap().clone())
            } else {
                sums.insert(sum, i as i32);
            }
        }
        max_len
    }

    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let l = s.len();
        if l < 2 { return s; }
        let sum = shift.iter().fold(0, |sum, val| sum + if val[0] == 0 { -val[1] } else { val[1] }) % l as i32;
        if sum == 0 { return s; }
        let mut res = String::with_capacity(l);
        let sh = i32::abs(sum) as usize;
        if sum > 0 {
            res.push_str(&s[l - sh..l]);
            res.push_str(&s[0..l - sh]);
        } else {
            res.push_str(&s[sh..l]);
            res.push_str(&s[0..sh]);
        }
        res
    }

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        if l == 0 { return vec![]; }
        if l == 1 { return vec![1]; }
        let mut left_products = vec![1i32; l];
        for i in 0..(l - 1) {
            left_products[i + 1] = left_products[i] * nums[i];
        }
        let mut right_products = vec![1i32; l];
        for i in (1..l).rev() {
            right_products[i - 1] = right_products[i] * nums[i];
        }
        left_products.iter().zip(right_products.iter()).map(|(lp, rp)| *lp * *rp).collect()
    }

    pub fn product_except_self_v2(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        if l == 0 { return vec![]; }
        if l == 1 { return vec![1]; }
        let mut res = vec![1i32; l];
        for i in 0..(l - 1) {
            res[i + 1] = res[i] * nums[i];
        }
        let mut right_product = 1;
        for i in (0..l).rev() {
            res[i] = right_product * res[i];
            right_product *= nums[i];
        }
        res
    }

    pub fn check_valid_string(s: String) -> bool {
        if s.is_empty() { return true; }
        let mut par_stack = Vec::<usize>::with_capacity(s.len());
        let mut ast_stack = Vec::<usize>::with_capacity(s.len());
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => { par_stack.push(i); },
                '*' => { ast_stack.push(i); },
                ')' if !par_stack.is_empty() => { par_stack.pop(); },
                ')' if par_stack.is_empty() && !ast_stack.is_empty() && i > ast_stack.pop().unwrap() => (),
                _ => return false,
            }
        }

        if par_stack.is_empty() { return true; }

        if par_stack.len() <= ast_stack.len() {
            while !par_stack.is_empty() {
                if par_stack.pop() > ast_stack.pop() { return false; }
            }
            return true;
        }
        false
    }
}

#[inline(always)]
pub fn apply_backspaces(iter: &mut Rev<Chars>) -> Option<char> {
    let mut ch = iter.next();
    let mut cnt = 0;
    loop {
        match ch {
            None => return None,
            Some('#') => { cnt += 1; ch = iter.next(); },
            Some(_) if cnt == 0 => return ch,
            _ => { cnt -= 1; ch = iter.next(); },
        }
    }
}