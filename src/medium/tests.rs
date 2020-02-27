use super::*;

#[test]
fn three_sum_test() {
    assert_eq!(Solution::three_sum(vec![]), vec![] as Vec<Vec<i32>>);
    assert_eq!(Solution::three_sum(vec![0]), vec![] as Vec<Vec<i32>>);
    assert_eq!(Solution::three_sum(vec![0,0,0]), vec![vec![0, 0, 0]]);
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}