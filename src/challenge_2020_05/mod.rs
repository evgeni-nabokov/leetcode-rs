#[cfg(test)]
mod tests;
mod trie;
mod trie_node;
mod list_node;

use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;
use std::cmp::{max, min};

use list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        if jewels.is_empty() || stones.is_empty() { return 0; }
        let mut cnt = 0;
        let mut j_set: HashSet<char> = HashSet::with_capacity(jewels.len());
        for j in jewels.chars() {
            j_set.insert(j);
        }
        for s in stones.chars() {
            if j_set.contains(&s) {
                cnt += 1;
            }
        }
        cnt
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > 0 && magazine.len() == 0 { return false; }
        let mut avail_chars = HashMap::<char, usize>::with_capacity(magazine.len());
        for c in magazine.chars() {
            *avail_chars.entry(c).or_insert(0) += 1;
        }
        for c in ransom_note.chars() {
            if let Entry::Occupied(mut o) = avail_chars.entry(c) {
                if *o.get() > 0 {
                    *o.get_mut() -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn can_construct_v2(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > 0 && magazine.len() == 0 { return false; }
        let mut chars = vec![0; 26];
        for c in magazine.chars() {
            chars[c as usize - 97] += 1;
        }
        for c in ransom_note.chars() {
            let i = c as usize - 97;
            if chars[i] == 0 {
                return false;
            } else {
                chars[i] -= 1;
            }
        }
        true
    }

    pub fn bitwise_complement(n: i32) -> i32 {
        let mut res = 0;
        let mut un = n as u32;
        let mut i = 0;

        loop {
            if un & 1 == 0 {
                res += 1 << i;
            }
            i += 1;
            un >>= 1;
            if un == 0 {
                break;
            }
        }
        res
    }

    pub fn bitwise_complement_v2(n: i32) -> i32 {
        max((n as u32 + 1).next_power_of_two() as i32, 2) - n - 1
    }

    pub fn first_uniq_char(s: String) -> i32 {
        if s.is_empty() { return -1; }
        let mut chars = vec![0; 26];
        for c in s.chars() {
            chars[c as usize - 97] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if chars[c as usize - 97] == 1 {
                return i as i32;
            }
        }
        -1
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counter_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len() / 2);
        for &n in nums.iter() {
            match counter_map.entry(n) {
                Entry::Occupied(o) if *o.get() >= nums.len() / 2 => {
                    return n;
                },
                Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                },
                Entry::Vacant(v) => {
                    v.insert(1);
                }
            }
        }
        *counter_map.keys().next().unwrap()
    }

    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        // 1 or 2 points are always belong to the same line.
        if coordinates.len() < 3 { return false; }
        // a * x + b * y + c = 0.
        // a = y1 - y2, b = x2 - x1, c = x1 * y2 - x2 * y1.
        let (x_1, y_1) = (coordinates[0][0], coordinates[0][1]);
        let (x_2, y_2) = (coordinates[1][0], coordinates[1][1]);
        let a = y_1 - y_2;
        let b = x_2 - x_1;
        let c = x_1 * y_2 - x_2 * y_1;
        // Check each point if it satisfies the equation of line above.
        for point in coordinates.iter().skip(2) {
            let (x, y) = (point[0], point[1]);
            if a * x + b * y + c != 0 {
                return false;
            }
        }
        true
    }

    pub fn is_perfect_square(num: i32) -> bool {
        let mut sum = 0;
        let mut odd_n = 1;
        loop {
            if sum == num - odd_n {
                return true;
            }
            if sum > num - odd_n {
                return false;
            }
            sum += odd_n;
            odd_n += 2;
        }
    }

    pub fn is_perfect_square_v2(num: i32) -> bool {
        if num == 1 { return true; }
        let mut x_prev = 1f64;
        let mut x = (num / 2) as f64;
        while (x_prev - x).abs() >= 1.0 {
            x_prev = x;
            x = (x + num as f64 / x) / 2.0;
        }
        let mut possible_roots = vec![x_prev.floor() as i32, x.floor() as i32];
        possible_roots.sort();
        possible_roots.dedup();
        for r in possible_roots {
            if r * r == num {
                return true
            }
        }
        false
    }

    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut who_trusts = vec![0; n as usize];
        let mut who_is_trusted = vec![0; n as usize];
        for t in trust.iter() {
            who_trusts[(t[0] - 1) as usize] += 1;
            who_is_trusted[(t[1] - 1) as usize] += 1;
        }
        let mut judge = -1i32;
        for i in 0..n as usize {
            if who_trusts[i] == 0 {
                if judge > 0 {
                    return -1;
                } else if who_is_trusted[i] == n - 1 {
                    judge = i as i32 + 1;
                } else {
                    return -1;
                }
            }
        }
        judge
    }

    pub fn find_judge_v2(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut counter = vec![0; n as usize];
        for t in trust.iter() {
            counter[(t[0] - 1) as usize] -= 1;
            counter[(t[1] - 1) as usize] += 1;
        }
        match counter.iter().position(|&p| p == n - 1) {
            Some(i) => i as i32 + 1,
            _ => -1,
        }
    }

    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        fn fill(image: &mut Vec<Vec<i32>>, r: usize, c: usize, new_color: i32, old_color: i32) {
            if image[r][c] != old_color || image[r][c] == new_color {
                return;
            }
            image[r][c] = new_color;
            if r > 0 {
                fill(image, r - 1, c, new_color, old_color);
            }
            if r < image.len() - 1 {
                fill(image, r + 1, c, new_color, old_color);
            }
            if c > 0 {
                fill(image, r, c - 1, new_color, old_color);
            }
            if c < image[0].len() - 1 {
                fill(image, r, c + 1, new_color, old_color);
            }
        }
        let old_color = image[sr as usize][sc as usize];
        fill(&mut image, sr as usize, sc as usize, new_color, old_color);
        image
    }

    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + ((right - left) / 2);
            if mid == 0 ||
                nums[mid] != nums[mid - 1] && nums[mid] != nums[mid + 1] {
                return nums[mid];
            }
            let start = if nums[mid] == nums[mid + 1] { mid } else { mid - 1 };
            if start % 2 == 0 {
                left = mid + 1;
            } else  {
                right = mid - 1;
            }
        }
        nums[left]
    }

    pub fn remove_k_digits(num: String, mut k: i32) -> String {
        if num.len() == k as usize { return "0".to_string(); }
        let mut res: Vec<char> = Vec::with_capacity(k as usize);
        let mut digits: Vec<char> = num.chars().collect();
        for d in digits.iter() {
            while k > 0 && !res.is_empty() && res.last().unwrap() > d {
                res.pop();
                k -= 1;
            }
            if !res.is_empty() || *d != '0' {
                res.push(*d);
            }
        }
        while k > 0 {
            res.pop();
            k -= 1;
        }
        while !res.is_empty() && res[0] == '0' {
            res.remove(0);
        }
        if res.is_empty() { "0".to_string() } else { res.iter().collect() }
    }

    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        let mut sum: i32 = nums[0];
        let mut sum_inv: i32 = -nums[0];
        let mut max_sum: i32 = sum;
        let mut max_sum_inv: i32 = sum_inv;
        let mut total_sum_inv = sum_inv;
        for &n in nums.iter().skip(1) {
            sum = max(n, sum + n);
            sum_inv = max(-n, sum_inv - n);
            max_sum = max(sum, max_sum);
            max_sum_inv = max(sum_inv, max_sum_inv);
            total_sum_inv -= n;
        }
        if max_sum < 0 {
            max_sum
        } else {
            max(max_sum, max_sum_inv - total_sum_inv)
        }
    }

    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }
        let mut odd_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(head.as_ref().unwrap().val)));
        if head.as_ref().unwrap().next.is_none() {
            return odd_head;
        }
        let mut even_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(head.as_ref().unwrap().next.as_ref().unwrap().val)));;
        let mut curr_odd_node: &mut Option<Box<ListNode>> = &mut odd_head;
        let mut curr_even_node: &mut Option<Box<ListNode>> = &mut even_head;
        let mut curr_node: &Option<Box<ListNode>> = &head.as_ref().unwrap().next.as_ref().unwrap().next;
        let mut pos = 1;
        while curr_node.is_some() {
            let new_node = Some(Box::new(ListNode::new(curr_node.as_ref().unwrap().val)));
            if pos % 2 == 0 {
                curr_even_node.as_mut().unwrap().next = new_node;
                curr_even_node = &mut curr_even_node.as_mut().unwrap().next;
            } else {
                curr_odd_node.as_mut().unwrap().next = new_node;
                curr_odd_node = &mut curr_odd_node.as_mut().unwrap().next;
            }
            curr_node = &curr_node.as_ref().unwrap().next;
            pos += 1;
        }
        curr_odd_node.as_mut().unwrap().next = even_head;
        odd_head
    }
}
