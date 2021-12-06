#[cfg(test)]
mod tests;

use std::collections::HashSet;

struct Solution;

impl Solution {
    // 2094. Finding 3-Digit Even Numbers.
    // https://leetcode.com/problems/finding-3-digit-even-numbers/
    // Time complexity: O(N^3).
    // Space complexity: O(1) - up to 450 elements in a temporary set.
    pub fn find_even_numbers(mut digits: Vec<i32>) -> Vec<i32> {
        digits.sort_unstable_by(|x, y| y.cmp(&x));

        let mut set = HashSet::new();
        let mut perm = vec![0; 6];

        for i in 0..digits.len() - 2 {
            if digits[i] == 0 {
                // Next digits are 0 since digits is sorted in descending order.
                break;
            }
            for j in (i + 1)..digits.len() - 1 {
                for k in (j + 1)..digits.len() {
                    let (a, b, c) = (digits[i], digits[j], digits[k]);
                    let n = a * 100 + b * 10 + c;
                    if !set.contains(&n) {
                        // N! permutations where N is number of digits.
                        // For N = 3 N! = 1 * 2 * 3 = 6.
                        // Permutations:
                        // a b c
                        // b a c
                        // b c a
                        // a c b
                        // c a b
                        // c b a

                        perm[0] = n;
                        perm[1] = b * 100 + a * 10 + c;
                        perm[2] = b * 100 + c * 10 + a;
                        perm[3] = a * 100 + c * 10 + b;
                        perm[4] = c * 100 + a * 10 + b;
                        perm[5] = c * 100 + b * 10 + a;

                        for p in 0..perm.len() {
                            if perm[p] >= 100 && perm[p] % 2 == 0 {
                                set.insert(perm[p]);
                            }
                        }
                    }
                }
            }
        }

        let mut res = set.into_iter().collect::<Vec<i32>>();
        res.sort_unstable();
        res
    }

    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn find_even_numbers_v2(digits: Vec<i32>) -> Vec<i32> {
        let mut counts = vec![0; 10];
        for i in 0..digits.len() {
            counts[digits[i] as usize] += 1;
        }

        let mut res = Vec::with_capacity(450);
        for n in (100..1000 as usize).step_by(2) {
            let ones = n % 10;
            let tens = n / 10 % 10;
            let hundreds = n / 100;

            counts[ones] -= 1;
            counts[tens] -= 1;
            counts[hundreds] -= 1;

            if counts[ones] >= 0 && counts[tens] >= 0 && counts[hundreds] >= 0 {
                res.push(n as i32);
            }

            counts[ones] += 1;
            counts[tens] += 1;
            counts[hundreds] += 1;
        }

        res
    }
}