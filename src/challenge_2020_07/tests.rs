use super::*;
use crate::challenge_2020_07::list_node::LinkedList;

#[test]
fn arrange_coins_test() {
    let test_cases = vec![
        (5, 2),
        (8, 3),
        (2146467959, 65519)
    ];
    for case in test_cases {
        assert_eq!(Solution::arrange_coins(case.0), case.1);
    }
}

#[test]
fn level_order_bottom_test() {
    let test_cases = vec![
        (
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            vec![
                vec![15, 7],
                vec![9, 20],
                vec![3],
            ]
        )
    ];
    for case in test_cases {
        assert_eq!(Solution::level_order_bottom(TreeNode::from_level_order(&case.0)), case.1);
    }
}

fn get_prison_after_n_days_test_cases() -> Vec<(Vec<i32>, i32, Vec<i32>)> {
    vec![
        (
            vec![],
            7,
            vec![],
        ),
        (
            vec![0, 1, 0, 1, 1, 0, 0, 1],
            0,
            vec![0, 1, 0, 1, 1, 0, 0, 1],
        ),
        (
            vec![0, 1, 0, 1, 1, 0, 0, 1],
            7,
            vec![0, 0, 1, 1, 0, 0, 0, 0],
        ),
        (
            vec![1, 0, 0, 1, 0, 0, 1, 0],
            1000000000,
            vec![0, 0, 1, 1, 1, 1, 1, 0],
        ),
        (
            vec![0, 0, 1, 1, 1, 1, 0, 0],
            8,
            vec![0,0,0,1,1,0,0,0]
        ),
        (
            vec![1, 1, 0, 1, 1, 0, 1, 1],
            6,
            vec![0, 0, 1, 0, 0, 1, 0, 0]
        ),
        (
            vec![1,0,0,1,0,0,0,1],
            826,
            vec![0,1,1,0,1,1,1,0]
        ),
    ]
}

#[test]
fn prison_after_n_days_test() {
    for case in get_prison_after_n_days_test_cases() {
        assert_eq!(Solution::prison_after_n_days(case.0, case.1), case.2);
    }
}

#[test]
fn prison_after_n_days_v2_test() {
    for case in get_prison_after_n_days_test_cases() {
        assert_eq!(Solution::prison_after_n_days_v2(case.0, case.1), case.2);
    }
}

#[test]
fn nth_ugly_number_test() {
    let test_cases = vec![
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 8),
        (8, 9),
        (9, 10),
        (10, 12),
    ];
    for case in test_cases {
        assert_eq!(Solution::nth_ugly_number(case.0), case.1);
    }
}

#[test]
fn hamming_distance_test() {
    let test_cases = vec![
        (1, 4, 2),
    ];
    for case in test_cases {
        assert_eq!(Solution::hamming_distance(case.0, case.1), case.2);
    }
}

#[test]
fn hamming_distance_v2_test() {
    let test_cases = vec![
        (1, 4, 2),
    ];
    for case in test_cases {
        assert_eq!(Solution::hamming_distance_v2(case.0, case.1), case.2);
    }
}

#[test]
fn hamming_distance_v3_test() {
    let test_cases = vec![
        (1, 4, 2),
    ];
    for case in test_cases {
        assert_eq!(Solution::hamming_distance_v3(case.0, case.1), case.2);
    }
}

#[test]
fn plus_one_test() {
    let test_cases = vec![
        (
            vec![0],
            vec![1],
        ),
        (
            vec![1, 2, 3],
            vec![1, 2, 4],
        ),
        (
            vec![4, 3, 2, 1],
            vec![4, 3, 2, 2],
        ),
        (
            vec![9],
            vec![1, 0],
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::plus_one(case.0), case.1);
    }
}

#[test]
fn island_perimeter_test() {
    let test_cases = vec![
        (
            vec![],
            0
        ),
        (
            vec![vec![0]],
            0
        ),
        (
            vec![vec![1]],
            4
        ),
        (
            vec![
                vec![0,1,0,0],
                vec![1,1,1,0],
                vec![0,1,0,0],
                vec![1,1,0,0],
            ],
            16
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::island_perimeter(case.0), case.1);
    }
}

fn get_three_sum_test_cases() -> Vec<(Vec<i32>, Vec<Vec<i32>>)>{
    vec![
        (vec![], vec![]),
        (vec![0], vec![]),
        (vec![0, 0, 0], vec![vec![0, 0, 0]]),
        (vec![-1, 0, 1, 2, -1, -4], vec![vec![-1, -1, 2], vec![-1, 0, 1]]),
    ]
}

#[test]
fn three_sum_test() {
    for case in get_three_sum_test_cases() {
        assert_eq!(Solution::three_sum(case.0), case.1);
    }
}

#[test]
fn three_sum_v2_test() {
    for case in get_three_sum_test_cases() {
        assert_eq!(Solution::three_sum_v2(case.0), case.1);
    }
}

#[test]
fn width_of_binary_tree_test() {
    let test_cases = vec![
        (
            vec![Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)],
            4
        ),
        (
            vec![Some(1), Some(3), None, Some(5), Some(3), None, None],
            2
        ),
        (
            vec![Some(1), Some(3), Some(2), Some(5), None, None, None],
            2
        ),
        (
            vec![Some(1), Some(3), Some(2), Some(5), None, None, Some(9), Some(6), None, None, None, None, None, None, Some(9)],
            8
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::width_of_binary_tree(TreeNode::from_level_order(&case.0)), case.1);
    }
}

#[test]
fn subsets_test() {
    let test_cases = vec![
        (
            vec![1, 2],
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
            ]
        ),
        (
            vec![1, 2, 3],
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ]
        ),
        (
            //vec![3,2,4,1],
            vec![1,2,3,4],
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![4],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 3, 4],
                vec![2, 3, 4],
                vec![1, 2, 3, 4],
            ]
        )
    ];
    for case in test_cases {
        assert_eq!(Solution::subsets(case.0), case.1);
    }
}

fn get_is_same_tree_test_cases() -> Vec<(Vec<Option<i32>>, Vec<Option<i32>>, bool)>{
    vec![
        (
            vec![],
            vec![Some(1)],
            false
        ),
        (
            vec![Some(1), Some(2), Some(3)],
            vec![Some(1), Some(2), Some(3)],
            true
        ),
        (
            vec![Some(1), Some(2)],
            vec![Some(1), None, Some(2)],
            false
        ),
        (
            vec![Some(1), Some(2), Some(1)],
            vec![Some(1), Some(1), Some(2)],
            false
        ),
    ]
}

#[test]
fn is_same_tree_test() {
    for case in get_is_same_tree_test_cases() {
        assert_eq!(Solution::is_same_tree(TreeNode::from_level_order(&case.0), TreeNode::from_level_order(&case.1)), case.2);
    }
}

#[test]
fn is_same_tree_v2_test() {
    for case in get_is_same_tree_test_cases() {
        assert_eq!(Solution::is_same_tree_v2(TreeNode::from_level_order(&case.0), TreeNode::from_level_order(&case.1)), case.2);
    }
}

#[test]
fn angle_clock_test() {
    let test_cases = vec![
        (12, 30, 165f64),
        (3, 30, 75f64),
        (4, 50, 155f64),
        (12, 0, 0f64),
        (3, 15, 7.5f64),
        (1, 57, 76.5f64),
    ];

    for case in test_cases {
        assert_eq!(Solution::angle_clock(case.0, case.1), case.2);
    }
}

#[test]
fn reverse_words_test() {
    let test_cases = vec![
        ("the sky is blue".to_string(), "blue is sky the".to_string()),
        ("  hello world!  ".to_string(), "world! hello".to_string()),
        ("a good   example".to_string(), "example good a".to_string()),
    ];

    for case in test_cases {
        assert_eq!(Solution::reverse_words(case.0), case.1);
    }
}

#[test]
fn my_pow_test() {
    let test_cases = vec![
        (2.00000, 10, 1024.00000),
        (2.10000, 3, 9.26100),
        (2.00000, -2, 0.25000),
        (0.00001, 2147483647, 0f64),
        (2.00000, -2147483648, 0f64)
    ];

    for case in test_cases {
        assert!(Solution::my_pow(case.0, case.1) - case.2 < 0.0001);
    }
}

#[test]
fn top_k_frequent_test() {
    let test_cases = vec![
        (vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]),
        (vec![1], 1, vec![1]),
    ];
    for case in test_cases {
        assert_eq!(Solution::top_k_frequent(case.0, case.1), case.2);
    }
}

#[test]
fn find_order_test() {
    let test_cases = vec![
        (2, vec![vec![1, 0]], vec![0, 1]),
        (4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]], vec![0, 1, 2, 3]),
        (2, vec![vec![1, 0], vec![0, 1]], vec![]),
        (4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2], vec![0, 3]], vec![]),
    ];

    for case in test_cases {
        assert_eq!(Solution::find_order(case.0, case.1), case.2);
    }
}

#[test]
fn add_binary_test() {
    let test_cases = vec![
        ("0".to_string(), "0".to_string(), "0".to_string()),
        ("1".to_string(), "0".to_string(), "1".to_string()),
        ("1".to_string(), "1".to_string(), "10".to_string()),
        ("11".to_string(), "1".to_string(), "100".to_string()),
        ("1010".to_string(), "1011".to_string(), "10101".to_string()),
        ("110010".to_string(), "10111".to_string(), "1001001".to_string()),
    ];
    for case in test_cases {
        assert_eq!(Solution::add_binary(case.0, case.1), case.2);
    }
}

#[test]
fn remove_elements_test() {
    let test_cases = vec![
        (vec![], 0, vec![]),
        (vec![1], 0, vec![1]),
        (vec![1], 1, vec![]),
        (vec![1, 1], 1, vec![]),
        (vec![1, 2], 1, vec![2]),
        (vec![1, 2, 3], 2, vec![1, 3]),
        (vec![1, 2, 6, 3, 4, 5, 6], 6, vec![1, 2, 3, 4, 5]),
    ];
    for case in test_cases {
        assert_eq!(Solution::remove_elements(ListNode::from_slice(&case.0), case.1).to_vec(), case.2);
    }
}

#[test]
fn exist_test() {
    let board = vec![
        vec!['A','B','C','E'],
        vec!['S','F','C','S'],
        vec!['A','D','E','E']
    ];

    let test_cases = vec![
        (
            "ABCCED".to_string(),
            true
        ),
        (
            "SEE".to_string(),
            true
        ),
        (
            "ABCB".to_string(),
            false
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::exist(board.clone(), case.0), case.1);
    }
}

#[test]
fn zigzag_level_order_test() {
    let test_cases = vec![
        (
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            vec![
                vec![3],
                vec![20, 9],
                vec![15, 7],
            ]
        ),
        (
            vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(5)],
            vec![
                vec![1],
                vec![3, 2],
                vec![4, 5],
            ]
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::zigzag_level_order(TreeNode::from_level_order(&case.0)), case.1);
    }
}