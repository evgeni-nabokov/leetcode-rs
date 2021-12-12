use super::*;

#[test]
fn build_array_test() {
    assert_eq!(Solution::build_array(vec![1, 3],  3), ["Push".to_string(), "Push".to_string(), "Pop".to_string(), "Push".to_string()]);
    assert_eq!(Solution::build_array(vec![1, 2, 3],  3), ["Push".to_string(), "Push".to_string(), "Push".to_string()]);
    assert_eq!(Solution::build_array(vec![1, 2],  4), ["Push".to_string(), "Push".to_string()]);
    assert_eq!(Solution::build_array(vec![2, 3, 4],  4), ["Push".to_string(), "Pop".to_string(), "Push".to_string(), "Push".to_string(), "Push".to_string()]);
}