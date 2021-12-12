use super::*;

#[test]
fn max_total_fruits_test() {
    let test_cases = vec![("B0B6G0R6R0R6G9", 1), ("B0R0G0R9R0B0G0", 1), ("G4", 0)];
    for case in test_cases {
        assert_eq!(Solution::count_points(case.0.to_string()), case.1)
    }
}
