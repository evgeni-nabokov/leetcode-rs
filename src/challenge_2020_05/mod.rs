#[cfg(test)]
mod tests;

use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;
use std::cmp::max;

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
}