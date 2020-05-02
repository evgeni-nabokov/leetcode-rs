use super::*;

#[test]
fn num_jewels_in_stones_test() {
    assert_eq!(Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()), 3);
    assert_eq!(Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
}