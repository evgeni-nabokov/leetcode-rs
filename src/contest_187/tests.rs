use super::*;

#[test]
fn dest_city_test() {
    assert_eq!(Solution::dest_city(vec![
        vec!["Moscow".to_string(), "Minsk".to_string()],
        vec!["Perm".to_string(), "Moscow".to_string()]
    ]), "Minsk".to_string());
}

#[test]
fn k_length_apart() {
    assert_eq!(Solution::k_length_apart(vec![1,0,0,0,1,0,0,1], 2), true);
    assert_eq!(Solution::k_length_apart(vec![1,0,0,1,0,1], 2), false);
    assert_eq!(Solution::k_length_apart(vec![0,1,0,1], 1), true);
    assert_eq!(Solution::k_length_apart(vec![1,1,1,1,1], 0), true);
    assert_eq!(Solution::k_length_apart(vec![0, 0, 0, 0], 1), true);
}