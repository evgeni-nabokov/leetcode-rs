use super::*;
use super::range_freq_query::RangeFreqQuery;

#[test]
fn max_distance_test() {
    let test_cases = vec![
        (vec![0, 1], 1),
        (vec![1, 1, 1, 6, 1, 1, 1], 3),
        (vec![1, 8, 3, 8, 3], 4),
    ];
    for case in test_cases {
        assert_eq!(Solution::max_distance(case.0), case.1);
    }
}

#[test]
fn watering_plants_test() {
    let test_cases = vec![
        (vec![2, 2, 3, 3], 5, 14),
        (vec![1, 1, 1, 4, 2, 3], 4, 30),
        (vec![7, 7, 7, 7, 7, 7, 7], 8, 49),
    ];
    for case in test_cases {
        assert_eq!(Solution::watering_plants(case.0, case.1), case.2);
    }
}

#[test]
fn range_freq_query_test() {
    let obj_1 = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
    assert_eq!(obj_1.query(1, 2, 4), 1);
    assert_eq!(obj_1.query(0, 11, 33), 2);

    let obj_2 = RangeFreqQuery::new(vec![1, 1, 1, 2, 2]);
    assert_eq!(obj_2.query(0, 1, 2), 0);
    assert_eq!(obj_2.query(0, 2, 1), 3);
    assert_eq!(obj_2.query(3, 3, 2), 1);
    assert_eq!(obj_2.query(2, 2, 1), 1);
}