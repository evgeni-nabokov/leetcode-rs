use super::*;

#[test]
fn single_number_test() {
    assert_eq!(Solution::single_number(vec![2,2,1]), 1);
    assert_eq!(Solution::single_number(vec![4,1,2,1,2]), 4);
}

#[test]
fn single_number_test2() {
    assert_eq!(Solution::single_number_v2(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number_v2(vec![4, 1, 2, 1, 2]), 4);
}