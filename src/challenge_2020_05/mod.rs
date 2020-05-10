#[cfg(test)]
mod tests;

use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;
use std::cmp::{max, min};

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
                Entry::Occupied(mut o) if *o.get() >= nums.len() / 2 => {
                    return n;
                },
                Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                },
                Entry::Vacant(mut v) => {
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

    pub fn is_perfect_square_v3(num: i32) -> bool {
        if num == 1 { return true; }
        let mut x_prev = 0f64;
        let mut x = (num / 2) as f64;
        while (x_prev - x).abs() >= 1.0 {
            x_prev = x;
            x = x - x / 2.0 + num as f64 / (2.0 * x)
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
}
