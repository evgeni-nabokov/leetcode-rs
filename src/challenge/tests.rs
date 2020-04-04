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

#[test]
fn move_zeroes_test() {
    let mut actual1 = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut actual1);
    let expected1 = vec![1, 3, 12, 0, 0];
    assert_eq!(actual1, expected1);

    let mut actual2 = vec![];
    Solution::move_zeroes(&mut actual2);
    let expected2 = vec![];
    assert_eq!(actual2, expected2);

    let mut actual3 = vec![0];
    Solution::move_zeroes(&mut actual3);
    let expected3 = vec![0];
    assert_eq!(actual3, expected3);

    let mut actual4 = vec![0, 1];
    Solution::move_zeroes(&mut actual4);
    let expected4 = vec![1, 0];
    assert_eq!(actual4, expected4);

    let mut actual5 = vec![0, 0, 0, 0];
    Solution::move_zeroes(&mut actual5);
    let expected5 = vec![0, 0, 0, 0];
    assert_eq!(actual5, expected5);

    let mut actual6 = vec![1, 1, 1, 1];
    Solution::move_zeroes(&mut actual6);
    let expected6 = vec![1, 1, 1, 1];
    assert_eq!(actual6, expected6);

    let mut actual7 = vec![0, -1, 0, -3, -12];
    Solution::move_zeroes(&mut actual7);
    let expected7 = vec![-1, -3, -12, 0, 0];
    assert_eq!(actual7, expected7);
}

#[test]
fn move_zeroes_test_2() {
    let mut actual1 = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes_v2(&mut actual1);
    let expected1 = vec![1, 3, 12, 0, 0];
    assert_eq!(actual1, expected1);

    let mut actual2 = vec![];
    Solution::move_zeroes_v2(&mut actual2);
    let expected2 = vec![];
    assert_eq!(actual2, expected2);

    let mut actual3 = vec![0];
    Solution::move_zeroes_v2(&mut actual3);
    let expected3 = vec![0];
    assert_eq!(actual3, expected3);

    let mut actual4 = vec![0, 1];
    Solution::move_zeroes_v2(&mut actual4);
    let expected4 = vec![1, 0];
    assert_eq!(actual4, expected4);

    let mut actual5 = vec![0, 0, 0, 0];
    Solution::move_zeroes_v2(&mut actual5);
    let expected5 = vec![0, 0, 0, 0];
    assert_eq!(actual5, expected5);

    let mut actual6 = vec![1, 1, 1, 1];
    Solution::move_zeroes_v2(&mut actual6);
    let expected6 = vec![1, 1, 1, 1];
    assert_eq!(actual6, expected6);

    let mut actual7 = vec![0, -1, 0, -3, -12];
    Solution::move_zeroes_v2(&mut actual7);
    let expected7 = vec![-1, -3, -12, 0, 0];
    assert_eq!(actual7, expected7);
}