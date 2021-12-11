#[cfg(test)]
mod tests;

use std::collections::HashSet;

struct Solution;

impl Solution {
    // 2062. Count Vowel Substrings of a String.
    // https://leetcode.com/problems/count-vowel-substrings-of-a-string/
    // Time complexity: O(N^2).
    // Space complexity: O(1).
    pub fn count_vowel_substrings(word: String) -> i32 {
        if word.len() < 5 {
            return 0;
        }

        let vowels: [u8; 5] = [97, 101, 105, 111, 117]; // ['a', 'e', 'i', 'o', 'u']
        let bytes = word.as_bytes();
        let mut set = HashSet::with_capacity(5);
        let mut res = 0;
        for left in 0..=bytes.len() - 5 {
            set.clear();
            for right in left..bytes.len() {
                if vowels.contains(&bytes[right]) {
                    set.insert(bytes[right]);
                } else {
                    break;
                }
                if set.len() == 5 {
                    res += 1;
                }
            }
        }

        res
    }

    // 2063. Vowels of All Substrings.
    // https://leetcode.com/problems/vowels-of-all-substrings/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn count_vowels(word: String) -> i64 {
        let vowels: [u8; 5] = [97, 101, 105, 111, 117]; // ['a', 'e', 'i', 'o', 'u']
        let l = word.len();
        let bytes = word.as_bytes();
        let mut res: i64 = 0;
        for i in 0..l {
            // For any vowel at index i, count the number of substrings that contain it:
            // You can pick the left bound of the substring to be anywhere from index 0...i
            // You can pick the right bound to be anywhere from i...n-1
            // Multiply the two to get the # of combinations of left/right bounds that contain this vowel.
            // Credit to Michel D'Sa.
            if vowels.contains(&bytes[i]) {
                res += (i + 1) as i64 * (l - i) as i64;
            }
        }

        res
    }

    // 2064. Minimized Maximum of Products Distributed to Any Store.
    // https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/
    // Time complexity: O(M * LogX), M - number of products, X - max quantity of a product.
    // Space complexity: O(1).
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        fn can_distribute(n: i32, quantities: &[i32], x: i32) -> bool {
            // Calculating the total number of stores to distribute all the products
            // with specific quantity for each store.
            let mut stores = 0;
            for &q in quantities {
                stores += q / x + if q % x > 0 { 1 } else { 0 };
                if stores > n {
                    return false;
                }
            }
            true
        }

        // Minimum of the possible answers (minimized maximum of quantity for a store).
        let mut left = 1;
        // Never need more than the largest quantity because at that point you can throw everything in 1 store per product type.
        let mut right = *quantities.iter().max().unwrap();

        while left < right {
            let mid = left + (right - left) / 2;
            if can_distribute(n, &quantities, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    // 2065. Maximum Path Quality of a Graph.
    // https://leetcode.com/problems/maximum-path-quality-of-a-graph/
    // DFS + Backtracking method.
    // Time complexity: ?
    // Space complexity: O(V * E), V - number of vertices, E - number of edges.
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        let n = values.len();
        let mut adj_list = vec![vec![]; n];
        for i in 0..edges.len() {
            let u = edges[i][0] as usize;
            let v = edges[i][1] as usize;
            let t = edges[i][2];
            adj_list[u].push((v, t));
            adj_list[v].push((u, t));
        }

        // Current value of the path.
        let mut curr_path_val = values[0];
        // Max value of a valid path.
        let mut max_path_val = values[0];
        // Number of times we've visited a node in the path.
        let mut freq = vec![0; n];
        freq[0] = 1;

        fn dfs(
            adj_list: &[Vec<(usize, i32)>],
            values: &[i32],
            curr: usize,
            time_left: i32,
            freq: &mut [i32],
            curr_path_val: &mut i32,
            max_path_val: &mut i32,
        ) {
            if curr == 0 {
                *max_path_val = *max_path_val.max(curr_path_val);
            }

            for &(next, cost) in &adj_list[curr] {
                if cost > time_left {
                    continue;
                }

                if freq[next] == 0 {
                    *curr_path_val += values[next];
                }
                freq[next] += 1;

                dfs(
                    adj_list,
                    values,
                    next,
                    time_left - cost,
                    freq,
                    curr_path_val,
                    max_path_val,
                );

                // Backtrack DFS.
                freq[next] -= 1;
                if freq[next] == 0 {
                    *curr_path_val -= values[next];
                }
            }
        }

        dfs(
            &adj_list,
            &values,
            0,
            max_time,
            &mut freq,
            &mut curr_path_val,
            &mut max_path_val,
        );

        max_path_val
    }
}
