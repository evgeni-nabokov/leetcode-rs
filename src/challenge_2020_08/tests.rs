use super::*;
use crate::challenge_2020_08::hash_set::HashSet;
use crate::challenge_2020_08::logger_v1::Logger as LoggerV1;
use crate::challenge_2020_08::logger_v2::Logger as LoggerV2;
use crate::challenge_2020_08::word_dictionary::WordDictionary;
use crate::common::linked_list::LinkedList;
use crate::common::tree_node::BinaryTree;

fn get_logger_test_cases<'a>() -> Vec<(i32, &'a str, bool)> {
    vec![
        (1, "foo", true),
        (2, "bar", true),
        (3, "foo", false),
        (8, "bar", false),
        (10, "foo", false),
        (11, "foo", true),
    ]
}

#[test]
fn logger_v1_test() {
    let mut logger = LoggerV1::new();
    for case in get_logger_test_cases() {
        assert_eq!(
            logger.should_print_message(case.0, case.1.to_string()),
            case.2
        );
    }
}

#[test]
fn logger_v2_test() {
    let mut logger = LoggerV2::new();
    for case in get_logger_test_cases() {
        assert_eq!(
            logger.should_print_message(case.0, case.1.to_string()),
            case.2
        );
    }
}

#[test]
fn detect_capital_use_test() {
    let test_cases = vec![
        ("USA".to_string(), true),
        ("USa".to_string(), false),
        ("leetcode".to_string(), true),
        ("LeetCode".to_string(), false),
        ("Google".to_string(), true),
        ("GooglE".to_string(), false),
    ];
    for case in test_cases {
        assert_eq!(Solution::detect_capital_use(case.0), case.1);
    }
}

#[test]
fn myhashset_test() {
    let mut set = HashSet::new();
    set.add(1);
    set.add(2);
    assert_eq!(set.contains(1), true);
    assert_eq!(set.contains(3), false);
    set.add(2);
    assert_eq!(set.contains(2), true);
    set.remove(2);
    assert_eq!(set.contains(2), false);
}

#[test]
fn is_palindrome_test() {
    let test_cases = vec![
        ("", true),
        ("a", true),
        ("aa", true),
        ("ab", false),
        ("a.", true),
        ("race a car", false),
        ("A man, a plan, a canal: Panama", true),
    ];

    for case in test_cases {
        assert_eq!(Solution::is_palindrome(case.0.to_string()), case.1);
    }
}

fn get_is_power_of_four_test_cases() -> Vec<(i32, bool)> {
    vec![
        (-4, false),
        (0, false),
        (1, true),
        (2, false),
        (3, false),
        (4, true),
        (5, false),
        (6, false),
        (7, false),
        (8, false),
        (9, false),
        (10, false),
        (12, false),
        (12, false),
        (13, false),
        (14, false),
        (15, false),
        (16, true),
        (17, false),
        (64, true),
        (65, false),
    ]
}

#[test]
fn is_power_of_four_test() {
    for case in get_is_power_of_four_test_cases() {
        assert_eq!(Solution::is_power_of_four(case.0), case.1);
    }
}

#[test]
fn is_power_of_four_v2_test() {
    for case in get_is_power_of_four_test_cases() {
        assert_eq!(Solution::is_power_of_four_v2(case.0), case.1);
    }
}

#[test]
fn is_power_of_four_v3_test() {
    for case in get_is_power_of_four_test_cases() {
        assert_eq!(Solution::is_power_of_four_v3(case.0), case.1);
    }
}

#[test]
fn word_dictionary_test() {
    let mut obj = WordDictionary::new();
    obj.add_word("bad".to_string());
    obj.add_word("dad".to_string());
    obj.add_word("mad".to_string());
    assert_eq!(obj.search("pad".to_string()), false);
    assert_eq!(obj.search("bad".to_string()), true);
    assert_eq!(obj.search(".ad".to_string()), true);
    assert_eq!(obj.search("b..".to_string()), true);
}

#[test]
fn find_duplicates_test() {
    let test_cases = vec![(vec![], vec![]), (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3])];
    for case in test_cases {
        assert_eq!(Solution::find_duplicates(case.0), case.1);
    }
}

#[test]
fn find_duplicates_v2_test() {
    let test_cases = vec![(vec![], vec![]), (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3])];
    for case in test_cases {
        assert_eq!(Solution::find_duplicates_v2(case.0), case.1);
    }
}

fn get_vertical_traversal_test_cases() -> Vec<(Vec<Option<i32>>, Vec<Vec<i32>>)> {
    vec![
        (
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            vec![vec![9], vec![3, 15], vec![20], vec![7]],
        ),
        (
            vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
            ],
            vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]],
        ),
    ]
}

#[test]
fn vertical_traversal_test() {
    for case in get_vertical_traversal_test_cases() {
        assert_eq!(
            Solution::vertical_traversal(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

#[test]
fn vertical_traversal_v2_test() {
    for case in get_vertical_traversal_test_cases() {
        assert_eq!(
            Solution::vertical_traversal_v2(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

fn get_closest_value_test_cases() -> Vec<(Vec<Option<i32>>, f64, i32)> {
    vec![
        (
            vec![Some(4), Some(2), Some(5), Some(1), Some(3)],
            3.714286,
            4,
        ),
        (vec![Some(4), Some(2), Some(5), Some(1), Some(3)], 1.1, 1),
        (vec![Some(1), None, Some(2)], 3.428571, 2),
        (vec![Some(8), Some(1)], 6.0, 8),
    ]
}

#[test]
fn closest_value_test() {
    for case in get_closest_value_test_cases() {
        assert_eq!(
            Solution::closest_value(TreeNode::from_level_order(&case.0), case.1),
            case.2
        );
    }
}

#[test]
fn closest_value_test_v2() {
    for case in get_closest_value_test_cases() {
        assert_eq!(
            Solution::closest_value_v2(TreeNode::from_level_order(&case.0), case.1),
            case.2
        );
    }
}

#[test]
fn path_sum_test() {
    let test_cases = vec![(
        vec![
            Some(10),
            Some(5),
            Some(-3),
            Some(3),
            Some(2),
            None,
            Some(11),
            Some(3),
            Some(-2),
            None,
            Some(1),
        ],
        8,
        3,
    )];
    for case in test_cases {
        assert_eq!(
            Solution::path_sum(TreeNode::from_level_order(&case.0), case.1),
            case.2
        );
    }
}

#[test]
fn oranges_rotting_test() {
    let test_cases = vec![
        (vec![], -1),
        (vec![vec![]], -1),
        (vec![vec![0]], 0),
        (vec![vec![1]], -1),
        (vec![vec![2]], 0),
        (vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]], 4),
        (vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]], -1),
    ];
    for case in test_cases {
        assert_eq!(Solution::oranges_rotting(case.0), case.1);
    }
}

#[test]
fn title_to_number_test() {
    let test_cases = vec![("A", 1), ("AB", 28), ("ZY", 701)];
    for case in test_cases {
        assert_eq!(Solution::title_to_number(case.0.to_string()), case.1);
    }
}

#[test]
fn get_row_test() {
    let test_cases = vec![
        (0, vec![1]),
        (1, vec![1, 1]),
        (2, vec![1, 2, 1]),
        (3, vec![1, 3, 3, 1]),
        (
            30,
            vec![
                1, 30, 435, 4060, 27405, 142506, 593775, 2035800, 5852925, 14307150, 30045015,
                54627300, 86493225, 119759850, 145422675, 155117520, 145422675, 119759850,
                86493225, 54627300, 30045015, 14307150, 5852925, 2035800, 593775, 142506, 27405,
                4060, 435, 30, 1,
            ],
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::get_row(case.0), case.1);
    }
}

#[test]
fn longest_palindrome_test() {
    let test_cases = vec![("abccccdd", 7), ("a", 1), ("bb", 2), ("aaaAaaaa", 7)];
    for case in test_cases {
        assert_eq!(Solution::longest_palindrome(case.0.to_string()), case.1);
    }
}

fn get_max_profit_test() -> Vec<(Vec<i32>, i32)> {
    vec![
        (vec![1], 0),
        (vec![3, 3, 5, 0, 0, 3, 1, 4], 6),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![7, 6, 4, 3, 1], 0),
    ]
}

#[test]
fn max_profit_test() {
    for case in get_max_profit_test() {
        assert_eq!(Solution::max_profit(case.0), case.1);
    }
}

#[test]
fn max_profit_v2_test() {
    for case in get_max_profit_test() {
        assert_eq!(Solution::max_profit_v2(case.0), case.1);
    }
}

#[test]
fn to_goat_latin_test() {
    let test_cases = vec![
        (
            "I speak Goat Latin",
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"),
        (
            "The quick brown fox jumped over the lazy dog",
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
        ),
    ];
    for case in test_cases {
        assert_eq!(
            Solution::to_goat_latin(case.0.to_string()),
            case.1.to_string()
        );
    }
}

#[test]
fn sort_array_by_parity_test() {
    let test_cases = vec![
        (vec![1], vec![1]),
        (vec![1, 2], vec![2, 1]),
        (vec![3, 1, 2, 4], vec![2, 4, 1, 3]),
    ];
    for case in test_cases {
        assert_eq!(Solution::sort_array_by_parity(case.0), case.1);
    }
}

#[test]
fn sort_array_by_parity_v2_test() {
    let test_cases = vec![
        (vec![1], vec![1]),
        (vec![1, 2], vec![2, 1]),
        (vec![3, 1, 2, 4], vec![2, 4, 3, 1]),
    ];
    for case in test_cases {
        assert_eq!(Solution::sort_array_by_parity_v2(case.0), case.1);
    }
}

#[test]
fn reorder_list_test() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![1], vec![1]),
        (vec![1, 2], vec![1, 2]),
        (vec![1, 2, 3], vec![1, 3, 2]),
        (vec![1, 2, 3, 4, 5], vec![1, 5, 2, 4, 3]),
    ];

    for case in test_cases {
        let mut head = ListNode::from_slice(&case.0);
        Solution::reorder_list(&mut head);
        assert_eq!(head.to_vec(), case.1);
    }
}

fn get_sum_of_left_leaves_test_cases() -> Vec<(Vec<Option<i32>>, i32)> {
    vec![
        (
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            24,
        ),
        (vec![Some(1), Some(2), Some(3), Some(4), Some(5)], 4),
    ]
}

#[test]
fn sum_of_left_leaves_test() {
    for case in get_sum_of_left_leaves_test_cases() {
        assert_eq!(
            Solution::sum_of_left_leaves(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

#[test]
fn sum_of_left_leaves_v2_test() {
    for case in get_sum_of_left_leaves_test_cases() {
        assert_eq!(
            Solution::sum_of_left_leaves_v2(TreeNode::from_level_order(&case.0)),
            case.1
        );
    }
}

fn get_has_path_test_cases() -> Vec<(Vec<Vec<i32>>, Vec<i32>, Vec<i32>, bool)> {
    let maze_1 = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
    ];

    let maze_2 = vec![
        vec![0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1],
        vec![0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1, 0, 0],
    ];

    vec![
        (maze_1.clone(), vec![0, 4], vec![4, 4], true),
        (maze_1.clone(), vec![0, 4], vec![3, 2], false),
        (maze_1.clone(), vec![0, 4], vec![4, 2], true),
        (maze_1.clone(), vec![0, 4], vec![1, 2], true),
        (maze_1.clone(), vec![0, 4], vec![0, 1], true),
        (maze_2.clone(), vec![0, 0], vec![8, 6], true),
    ]
}

#[test]
fn has_path_test() {
    for case in get_has_path_test_cases() {
        assert_eq!(Solution::has_path(case.0.clone(), case.1, case.2), case.3);
    }
}

#[test]
fn has_path_v2_test() {
    for case in get_has_path_test_cases() {
        assert_eq!(
            Solution::has_path_v2(case.0.clone(), case.1, case.2),
            case.3
        );
    }
}

#[test]
fn fizz_buzz_test() {
    let test_cases = vec![(
        15,
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ],
    )];

    for case in test_cases {
        assert_eq!(Solution::fizz_buzz(case.0), case.1);
    }
}

#[test]
fn find_right_interval_test() {
    let test_cases = vec![
        (vec![vec![1, 2]], vec![-1]),
        (vec![vec![3, 4], vec![2, 3], vec![1, 2]], vec![-1, 0, 1]),
        (vec![vec![1, 4], vec![2, 3], vec![3, 4]], vec![-1, 2, -1]),
    ];

    for case in test_cases {
        assert_eq!(Solution::find_right_interval(case.0), case.1);
    }
}

#[test]
fn delete_node_test() {
    let empty_tree = vec![];
    let tree_1 = vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)];
    let tree_2 = vec![
        Some(50),
        Some(30),
        Some(70),
        None,
        Some(40),
        Some(60),
        Some(80),
    ];
    // TODO: Make this case working.
    // let tree_3_input = vec![Some(2),Some(0),Some(33),None,Some(1),Some(25),Some(40),None,
    //                   None,Some(11),Some(31),Some(34),Some(45),Some(10),Some(18),Some(29),Some(32),
    //                   None,Some(36),Some(43),Some(46),Some(4),None,Some(12),Some(24),Some(26),Some(30),
    //                   None,None,Some(35),Some(39),Some(42),Some(44),None,Some(48),Some(3),Some(9),None,
    //                   Some(14),Some(22),None,None,Some(27),None,None,None,None,Some(38),None,Some(41),
    //                   None,None,None,Some(47),Some(49),None,None,Some(5),None,Some(13),Some(15),Some(21),
    //                   Some(23),None,Some(28),Some(37),None,None,None,None,None,None,None,None,Some(8),
    //                   None,None,None,Some(17),Some(19),None,None,None,None,None,None,None,Some(7),None,
    //                   Some(16),None,None,Some(20),Some(6)];
    // let tree_3_output = vec![Some(2),Some(0),Some(34),None,Some(1),Some(25),Some(40),
    //                          None,None,Some(11),Some(31),Some(35),Some(45),Some(10),Some(18),Some(29),
    //                          Some(32),None,Some(36),Some(43),Some(46),Some(4),None,Some(12),Some(24),
    //                          Some(26),Some(30),None,None,None,Some(39),Some(42),Some(44),None,Some(48),
    //                          Some(3),Some(9),None,Some(14),Some(22),None,None,Some(27),None,None,Some(38),
    //                          None,Some(41),None,None,None,Some(47),Some(49),None,None,Some(5),None,Some(13),
    //                          Some(15),Some(21),Some(23),None,Some(28),Some(37),None,None,None,None,None,
    //                          None,None,None,Some(8),None,None,None,Some(17),Some(19),None,None,None,None,
    //                          None,None,None,Some(7),None,Some(16),None,None,Some(20),Some(6)];

    let test_cases = vec![
        (&empty_tree, 0, vec![]),
        (
            &tree_1,
            3,
            vec![Some(5), Some(4), Some(6), Some(2), None, None, Some(7)],
        ),
        (
            &tree_1,
            3,
            vec![Some(5), Some(4), Some(6), Some(2), None, None, Some(7)],
        ),
        (
            &tree_1,
            0,
            vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)],
        ),
        (
            &tree_1,
            2,
            vec![Some(5), Some(3), Some(6), None, Some(4), None, Some(7)],
        ),
        (
            &tree_1,
            4,
            vec![Some(5), Some(3), Some(6), Some(2), None, None, Some(7)],
        ),
        (
            &tree_1,
            6,
            vec![Some(5), Some(3), Some(7), Some(2), Some(4)],
        ),
        (
            &tree_1,
            7,
            vec![Some(5), Some(3), Some(6), Some(2), Some(4)],
        ),
        (
            &tree_1,
            5,
            vec![Some(6), Some(3), Some(7), Some(2), Some(4)],
        ),
        (
            &tree_2,
            50,
            vec![Some(60), Some(30), Some(70), None, Some(40), None, Some(80)],
        ),
        // (
        //     &tree_3_input,
        //     33,
        //     tree_3_output
        // )
    ];
    for case in test_cases {
        assert_eq!(
            Solution::delete_node(TreeNode::from_level_order(case.0), case.1)
                .get_level_order_values(),
            case.2
        );
    }
}
