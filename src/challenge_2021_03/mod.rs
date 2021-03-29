#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 1198. Find Smallest Common Element in All Rows.
    // https://leetcode.com/problems/find-smallest-common-element-in-all-rows/
    // Time complexity: O(Rows x Cols).
    // Space complexity: O(C).
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let mut freq_vec = vec![0usize; 10001];
        for row in 0..mat.len() {
            for col in 0..mat[0].len() {
                let i = mat[row][col] as usize;
                freq_vec[i] += 1;
                if freq_vec[i] == mat.len() {
                    return mat[row][col];
                }
            }
        }
        -1
    }

    // Time complexity: O(Rows x Cols x Log (Cols)).
    // Space complexity: O(C).
    pub fn smallest_common_element_v2(mat: Vec<Vec<i32>>) -> i32 {
        for col in 0..mat[0].len() {
            let mut found = false;
            for row in 1..mat.len() {
                if mat[row].binary_search(&mat[0][col]).is_ok() {
                    found = true;
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                return mat[0][col];
            }
        }
        -1
    }
}