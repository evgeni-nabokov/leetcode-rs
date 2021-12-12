#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 2103. Rings and Rods.
    // https://leetcode.com/problems/rings-and-rods/
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn count_points(rings: String) -> i32 {
        let mut color_counter = vec![vec![false; 3]; 10];
        for ring in rings.as_bytes().chunks(2) {
            let rod = (ring[1] - b'0') as usize;
            let color = match ring[0] {
                b'R' => 0,
                b'G' => 1,
                b'B' => 2,
                _ => unreachable!(),
            };
            color_counter[rod][color] = true;
        }

        color_counter
            .into_iter()
            .filter(|x| x.into_iter().all(|y| *y))
            .count() as i32
    }

    // 2104. Sum of Subarray Ranges.
    // https://leetcode.com/problems/sum-of-subarray-ranges/
    // Time complexity: O(N^2).
    // Space complexity: O(1).
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        for left in 0..nums.len() - 1 {
            let mut min = nums[left];
            let mut max = nums[left];
            for right in left + 1..nums.len() {
                min = min.min(nums[right]);
                max = max.max(nums[right]);
                res += (max - min) as i64;
            }
        }
        res
    }
}
