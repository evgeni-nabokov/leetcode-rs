use crate::common::tree_node::TreeNode;
use crate::tree::Solution;

fn get_has_path_sum_test_cases() -> Vec<(Vec<Option<i32>>, i32, bool)> {
    vec![
        (vec![], 0, false),
        (
            vec![
                Some(5),
                Some(4),
                Some(8),
                Some(11),
                None,
                Some(13),
                Some(4),
                Some(7),
                Some(2),
                None,
                None,
                None,
                Some(1),
            ],
            22,
            true,
        ),
        (vec![Some(1), Some(2), Some(3)], 5, false),
        (vec![Some(1), Some(2)], 1, false),
    ]
}

#[test]
fn has_path_sum_test() {
    for case in get_has_path_sum_test_cases() {
        assert_eq!(
            Solution::has_path_sum(TreeNode::from_level_order(&case.0), case.1),
            case.2
        );
    }
}

#[test]
fn has_path_sum_v2_test() {
    for case in get_has_path_sum_test_cases() {
        assert_eq!(
            Solution::has_path_sum_v2(TreeNode::from_level_order(&case.0), case.1),
            case.2
        );
    }
}
