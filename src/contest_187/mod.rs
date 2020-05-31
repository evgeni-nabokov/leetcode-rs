#[cfg(test)]
mod tests;

use std::cmp::max;
use std::collections::{HashSet, BTreeMap};
use std::collections::btree_map::Entry::Occupied;

struct Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut origin_city_set = HashSet::with_capacity(paths.len());
        let mut dest_city_set = HashSet::with_capacity(paths.len());
        for edge in paths.iter() {
            origin_city_set.insert(&edge[0]);
            dest_city_set.insert(&edge[1]);
        }
        for city in origin_city_set.iter() {
            dest_city_set.remove(city);
        }
        dest_city_set.iter().next().cloned().cloned().unwrap()
    }

    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut dist = -1;
        for &n in nums.iter() {
            if n == 0 {
                if dist >= 0 {
                    dist += 1;
                }
            } else {
                if dist < 0 {
                    dist = 0;
                } else {
                    if dist < k {
                        return false
                    } else {
                        dist = 0;
                    }
                }
            }
        }
        true
    }

    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut left = 0usize;
        let mut max_dist = 0usize;
        let mut multiset: BTreeMap<i32, usize> = BTreeMap::new();
        for right in 0..nums.len() {
            *multiset.entry(nums[right]).or_insert(0) += 1;
            let mut min_n = *multiset.iter().next().unwrap().0;
            let mut max_n = *multiset.iter().next_back().unwrap().0;
            if max_n - min_n <= limit {
                max_dist = max(max_dist, right - left + 1);
            } else {
                while left <= right && max_n - min_n > limit {
                    match multiset.entry(nums[left]) {
                        Occupied(mut entry) => {
                            *entry.get_mut() -= 1;
                            if *entry.get() == 0 {
                                entry.remove();
                            }
                        },
                        _ => (),
                    };
                    min_n = *multiset.iter().next().unwrap().0;
                    max_n = *multiset.iter().next_back().unwrap().0;
                    left += 1;
                }
            }
        }
        max_dist as i32
    }
}
