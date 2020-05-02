use super::*;

#[test]
fn max_score_test() {
    assert_eq!(Solution::max_score("011101".to_string()), 5);
    assert_eq!(Solution::max_score("00111".to_string()), 5);
    assert_eq!(Solution::max_score("1111".to_string()), 3);
}

// #[test]
// fn find_diagonal_order() {
//     assert_eq!(Solution::find_diagonal_order(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]), vec![1,4,2,7,5,3,8,6,9]);
//     assert_eq!(Solution::find_diagonal_order(vec![vec![1,2,3,4,5],vec![6,7],vec![8],vec![9,10,11],vec![12,13,14,15,16]]), vec![1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]);
//     assert_eq!(Solution::find_diagonal_order(vec![vec![1,2,3,4,5,6]]), vec![1,2,3,4,5,6]);
// }
