use super::*;

#[test]
fn single_number_test() {
    assert_eq!(Solution::single_number(vec![2,2,1]), 1);
    assert_eq!(Solution::single_number(vec![4,1,2,1,2]), 4);
}

#[test]
fn single_number_test_v2() {
    assert_eq!(Solution::single_number_v2(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number_v2(vec![4, 1, 2, 1, 2]), 4);
}

#[test]
fn happy_number_test() {
    assert_eq!(Solution::is_happy(19), true);
}

#[test]
fn max_sub_array_test() {
    assert_eq!(Solution::max_sub_array(vec![0]), 0);
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    assert_eq!(Solution::max_sub_array(vec![-1,-2,-3,-4]), -1);
    assert_eq!(Solution::max_sub_array(vec![-2,-1,-3,-4]), -1);
    assert_eq!(Solution::max_sub_array(vec![-2,-3,-4,-1]), -1);
    assert_eq!(Solution::max_sub_array(vec![1,-2,-3,-4]), 1);
    assert_eq!(Solution::max_sub_array(vec![-2,1,-3,-4]), 1);
    assert_eq!(Solution::max_sub_array(vec![-2,-3,-4,1]), 1);

    assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    assert_eq!(Solution::max_sub_array(vec![10,1,-3,4,-1,2,1,-5,4]), 14);
    assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-3,2,1,-5,4]), 4);
}