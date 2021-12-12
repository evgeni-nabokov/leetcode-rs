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

    // 2105. Watering Plants II.
    // https://leetcode.com/problems/watering-plants-ii/
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let mut res = 0;
        let mut water_in_can_a = capacity_a;
        let mut water_in_can_b = capacity_b;
        let mut left = 0;
        let mut right = plants.len() - 1;
        while left < right {
            if water_in_can_a < plants[left] {
                res += 1;
                water_in_can_a = capacity_a;
            }
            water_in_can_a -= plants[left];
            left += 1;
            if water_in_can_b < plants[right] {
                res += 1;
                water_in_can_b = capacity_b;
            }
            water_in_can_b -= plants[right];
            right -= 1;
        }

        if left == right && plants[left] > water_in_can_a.max(water_in_can_b) {
            res += 1;
        }

        res
    }
}
