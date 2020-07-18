#[cfg(test)]
mod tests;

use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    // https://leetcode.com/problems/string-matching-in-an-array/
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        if words.len() < 2 { return Vec::new(); }
        let mut res = Vec::<String>::with_capacity(100);
        words.sort_unstable_by(|x, y| x.len().cmp(&y.len()));
        let mut skip = 1;
        for sub_word in &words {
            for word in words.iter().skip(skip) {
                if sub_word.len() != word.len() {
                    if word.contains(sub_word) {
                        res.push(sub_word.clone());
                        break;
                    }
                }
            }
            skip += 1;
        }
        res
    }

    // https://leetcode.com/problems/html-entity-parser/
    pub fn entity_parser(text: String) -> String {
        let mut res = Vec::<char>::with_capacity(text.len());
        let entity_map: HashMap<&str, char> =
            vec![("quot", '"'), ("apos", '\''), ("amp", '&'), ("gt", '>'), ("lt", '<'), ("frasl", '/')]
                .into_iter().collect();
        let mut entity_vec = Vec::<char>::with_capacity(5);
        let mut is_collecting_entity = false;
        for c in text.chars() {
            if c == '&' {
                is_collecting_entity = true;
                entity_vec.clear();
            } else if c == ';' && is_collecting_entity {
                is_collecting_entity = false;
                let key = entity_vec.iter().collect::<String>();
                if entity_map.contains_key(key.as_str()) {
                    res.push(entity_map.get(key.as_str()).unwrap().clone());
                } else {
                    res.push('&');
                    res.extend_from_slice(&entity_vec);
                    res.push(';');
                }
            } else if is_collecting_entity {
                if c.is_ascii_alphabetic() {
                    entity_vec.push(c);
                } else {
                    is_collecting_entity = false;
                }
            } else {
                res.push(c);
            }
        }
        res.into_iter().collect()
    }

    // https://leetcode.com/problems/queries-on-a-permutation-with-key/
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        if queries.len() == 0 || m == 0 { return Vec::new(); }
        let mut res: Vec<i32> = Vec::with_capacity(queries.len());
        let mut p_list: VecDeque<i32> = VecDeque::with_capacity(m as usize);
        for i in 1..=m {
            p_list.push_back(i);
        }
        for q in queries {
            for (i, &p) in p_list.iter().enumerate() {
                if q == p {
                    res.push(i as i32);
                    p_list.remove(i);
                    p_list.push_front(q);
                    break;
                }
            }
        }
        res
    }

    // https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/
    pub fn num_of_ways(n: i32) -> i32 {
        if n == 0 { return 1; }
        if n == 1 { return 12; }
        let modulo = 1000000007u64;
        let mut x = 6u64; // three color variants for n = 1.
        let mut y = 6u64; // two color variants for n = 1.
       for _ in 2..=n {
            let old_x = x;
            x = (2 * x + 2 * y) % modulo;
            y = (2 * old_x + 3 * y) % modulo;
        }
        ((x + y) % modulo) as i32
    }
}