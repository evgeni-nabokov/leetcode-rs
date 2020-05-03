use std::collections::{HashSet, HashMap};
use std::cmp::{min, max};

#[cfg(test)]
mod tests;

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
}
