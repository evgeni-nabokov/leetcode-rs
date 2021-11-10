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
}