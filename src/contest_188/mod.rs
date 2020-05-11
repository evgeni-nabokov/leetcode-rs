#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut pos = 1;
        let mut res: Vec<String> = Vec::with_capacity(n as usize);
        for &t in target.iter() {
            while pos < t {
                res.push("Push".to_string());
                res.push("Pop".to_string());
                pos += 1;
            }
            res.push("Push".to_string());
            pos += 1;
        }
        res
    }
}