use super::*;
use super::pick_index;
use crate::common::tree_node::{BinaryTree, TreeNode};
use crate::challenge_2020_06::randomized_set::{RandomizedSet as RandomizedSet};
use crate::challenge_2020_06::randomized_set_v2::{RandomizedSet as RandomizedSetV2};

#[test]
fn invert_tree_test() {
    let test_cases = vec![
        (
            vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)],
            vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)]
        ),
        (
            vec![Some(1), Some(2)],
            vec![Some(1), None, Some(2)],
        ),
    ];
    for case in test_cases {
        let tree = TreeNode::create_from_level_order(&case.0);
        assert_eq!(Solution::invert_tree(tree).get_level_order_values(), case.1);
    }
}

fn get_two_city_sched_cost_test_cases() -> Vec<(Vec<Vec<i32>>, i32)>{
    vec![
        (vec![vec![10,10], vec![20,20], vec![30,30], vec![40,40], vec![50,50]], 150),
        (vec![vec![10,20], vec![30,200], vec![400,50], vec![30,20]], 110),
        (vec![vec![20,10], vec![200,30], vec![400,50], vec![30,20]], 130),
    ]
}

#[test]
fn two_city_sched_cost_test() {
    for case in get_two_city_sched_cost_test_cases() {
        assert_eq!(Solution::two_city_sched_cost(case.0), case.1);
    }
}

#[test]
fn reverse_string_test() {
    let test_cases = vec![
        (
            vec![],
            vec![],
        ),
        (
            vec!['h', 'e', 'l', 'l', 'o'],
            vec!['o', 'l', 'l', 'e', 'h'],
        ),
    ];
    for mut case in test_cases {
        Solution::reverse_string(&mut case.0);
        assert_eq!(case.0, case.1);
    }
}

// #[test]
fn pick_index_test() {
    let mut obj = pick_index::Solution::new(vec![1]);
    println!("{}", obj.pick_index());

    obj = pick_index::Solution::new(vec![1, 3]);
    println!("{}", obj.pick_index());
    println!("{}", obj.pick_index());
    println!("{}", obj.pick_index());
    println!("{}", obj.pick_index());
    println!("{}", obj.pick_index());
}

fn get_reconstruct_queue_test_cases() -> Vec<(Vec<Vec<i32>>, Vec<Vec<i32>>)>{
    vec![
        (
            vec![vec![3, 0], vec![4, 2], vec![6, 1], vec![8, 0]],
            vec![vec![3, 0], vec![8, 0], vec![6, 1], vec![4, 2]],
        ),
        (
            vec![vec![7, 0], vec![4, 4], vec![7, 1], vec![5, 0], vec![6, 1], vec![5, 2]],
            vec![vec![5, 0], vec![7, 0], vec![5, 2], vec![6, 1], vec![4, 4], vec![7, 1]],
        ),
    ]
}

#[test]
fn reconstruct_queue_test() {
    for case in get_reconstruct_queue_test_cases() {
        assert_eq!(Solution::reconstruct_queue(case.0), case.1);
    }
}

#[test]
fn reconstruct_queue_v2_test() {
    for case in get_reconstruct_queue_test_cases() {
        assert_eq!(Solution::reconstruct_queue_v2(case.0), case.1);
    }
}

fn get_change_test_cases() -> Vec<(i32, Vec<i32>, i32)>{
    vec![
        (1, vec![], 0),
        (0, vec![1, 2], 1),
        (1, vec![1, 2], 1),
        (2, vec![1, 2], 2),
        (2, vec![2], 1),
        (10, vec![2], 1),
        (3, vec![1, 2], 2),
        (5, vec![1, 2, 5], 4),
        (500, vec![2, 7, 13], 717),
    ]
}

#[test]
fn change_test() {
    for case in get_change_test_cases() {
        assert_eq!(Solution::change(case.0, case.1), case.2);
    }
}

fn get_is_power_of_two_test_cases() -> Vec<(i32, bool)>{
    vec![
        (0, false),
        (1, true),
        (2, true),
        (16, true),
        (-16, false),
        (218, false),
        (-218, false),
        (-2147483648, false),
    ]
}

#[test]
fn is_power_of_two_test() {
    for case in get_is_power_of_two_test_cases() {
        assert_eq!(Solution::is_power_of_two(case.0), case.1);
    }
}

#[test]
fn is_power_of_two_v2_test() {
    for case in get_is_power_of_two_test_cases() {
        assert_eq!(Solution::is_power_of_two_v2(case.0), case.1);
    }
}

#[test]
fn is_power_of_two_v3_test() {
    for case in get_is_power_of_two_test_cases() {
        assert_eq!(Solution::is_power_of_two_v3(case.0), case.1);
    }
}

#[test]
fn is_subsequence_test() {
    let test_cases = vec![
        (
            "".to_string(),
            "".to_string(),
            true
        ),
        (
            "a".to_string(),
            "".to_string(),
            false
        ),
        (
            "".to_string(),
            "a".to_string(),
            true
        ),
        (
            "abc".to_string(),
            "ahbgdc".to_string(),
            true
        ),
        (
            "axc".to_string(),
            "ahbgdc".to_string(),
            false
        ),
    ];
    for case in test_cases {
        assert_eq!(Solution::is_subsequence(case.0, case.1), case.2);
    }
}

#[test]
fn search_insert_test() {
    let test_cases = vec![
        (vec![], 5, 0),
        (vec![1,3,5,6], 5, 2),
        (vec![1,3,5,6], 2, 1),
        (vec![1,3,5,6], 7, 4),
        (vec![1,3,5,6], 0, 0),
    ];
    for case in test_cases {
        assert_eq!(Solution::search_insert(case.0, case.1), case.2);
    }
}

#[test]
fn sort_colors_test() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![0], vec![0]),
        (vec![2], vec![2]),
        (vec![0, 1], vec![0, 1]),
        (vec![2, 2], vec![2, 2]),
        (vec![1, 1], vec![1, 1]),
        (vec![0, 1, 2], vec![0, 1, 2]),
        (vec![2, 1, 0], vec![0, 1, 2]),
        (vec![2, 0, 2, 1, 1, 0], vec![0, 0, 1, 1, 2, 2]),
        (vec![2, 0, 1], vec![0, 1, 2]),
        (vec![1,2,0], vec![0, 1, 2]),
        (vec![2,0,2,1,1,0], vec![0,0,1,1,2,2]),
    ];
    for mut case in test_cases {
        Solution::sort_colors(&mut case.0);
        assert_eq!(case.0, case.1);
    }
}

#[test]
fn randomized_set_test() {
    let mut s = RandomizedSet::new();
    assert_eq!(s.insert(0), true);
    assert_eq!(s.insert(1), true);
    assert_eq!(s.remove(0), true);
    assert_eq!(s.insert(2), true);
    assert_eq!(s.remove(1), true);
    assert_eq!(s.get_random(), 2);
}

#[test]
fn randomized_set_v2_test() {
    let mut s = RandomizedSetV2::new();
    assert_eq!(s.insert(0), true);
    assert_eq!(s.insert(1), true);
    assert_eq!(s.remove(0), true);
    assert_eq!(s.insert(2), true);
    assert_eq!(s.remove(1), true);
    assert_eq!(s.get_random(), 2);
}

#[test]
fn largest_divisible_subset_test() {
    let test_cases = vec![
        (vec![1, 2, 3], vec![1, 2]),
        (vec![1, 2, 4, 8], vec![1, 2, 4, 8]),
        (vec![1, 2, 3, 4, 6, 24], vec![1, 2, 4, 24]),
        (
            vec![889,27,652,25,468,164,417,98,163,564,4,287,586,947,138,753,756,105,975,993,203,879,775,764,229,448,132,473,529,751,988,777,992,793,596,155,210,45,318,125,49,956,62,347,93,167,454,549,141,447,103,371,573,866,599,372,349,71,900,451,411,317,579,402,461,175,908,22,219,577,147,619,532,560,976,726,771,876,218,569,979,588,196,186,512,897,609,763,79,926,124,892,452,31,913,608,641,491,971,309,819,48,694,266,999,24,853,220,391,2,351,117,814,721,547,91,438,542,226,185,113,556,676,83,864,846,131,295,539,748,645,915,600,136,134,342,489,601,692,605,766,152,857,959,589,750,291,313,369,404,396,838,436,241,320,730,67,46,19,861,531,360,350,483,52,954,248,457,456,126,398,558,310,688,222,30,443,442,778,633,877,370,952,958,683,898,741,896,780,245,262,503,550,862,359,669,808,795,189,238,966,469,525,426,540,895,460,790,987,355,439,951,183,293,314,211,64,642,267,112,89,701,681,773,922,149,574,352,693,385,680,92,709,470,394,243,762,598,638,428,54,887,477,695,506,135,289,815,85,563,963,9,888,53,624,740,732,715,800,553,43,315,58,742,733,980,779,837,634,275,41,834,197,280,433,249,904,172,981,970,323,885,430,365,199,840,397,292,409,481,102,260,88,524,961,362,603,546,643,731,746,923,994,178,686,326,8,929,725,761,768,176,953,28,629,839,716,799,429,690,344,978,684,723,717,625,42,282,252,329,181,769,504,485,796,484,200,177,128,860,554,32,170,493,304,118,142,937,745,498,810,109,826,165,874,658,361,537,921,749,36,918,319,788,628,373,50,833,216,94,802,339,202,985,237,356,655,724,682,75,500,388,321,308,405,393,482,499,635,973,544,296,884,535,607,977,538,10,384,389,917,957,368,180,425,169,850,816,160,51,264,378,240,268,602,656,697,610,659,919,708,528,253,881,215,392,831,593,17,646,332,159,941,353,859,82,534,29,414,281,813,734,377,567,156,15,400,832,166,718,647,471,380,322,948,797,14,254,399,5,354,872,938,906,305,496,303,34,35,518,673,817,336,557,613,37,421,231,236,424,490,406,146,420,805,572,441,257,445,221,886,198,893,789,341,100,182,505,327,758,960,909,121,122,729,949,209,630,996,711,786,145,650,570,848,759,440,7,153,912,302,450,894,232,791,462,772,217,767,263,543,552,787,408,358,519,755,754,407,565,653,858,699,1000,346,720,566,81,662,674,239,73,60,179,193,111,143,616,760,348,99,792,476,812,871,523,585,357,338,962,882,820,474,555,677,488,561,212,841,69,1,950,250,983,70,234,639,580,794,559,190,945,752,843,935,247,705,328,722,873,863,672,902,637,623,578,453,910,984,844,230,171,823,271,943,928,782,72,376,520,495],
            vec![1,2,4,8,32,64,128,384,768],
        )
    ];
    for case in test_cases {
        let mut res = Solution::largest_divisible_subset(case.0);
        res.sort_unstable();
        assert_eq!(res, case.1);
    }
}

#[test]
fn find_cheapest_price_test() {
    let test_cases = vec![
        (3, vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]], 0, 2, 1, 200),
        (3, vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]], 0, 2, 0, 500),
        (5, vec![vec![4,1,1],vec![1,2,3],vec![0,3,2],vec![0,4,10],vec![3,1,1],vec![1,4,3]], 2, 1, 1, -1),
    ];
    for case in test_cases {
        assert_eq!(Solution::find_cheapest_price(case.0, case.1, case.2, case.3, case.4), case.5);
    }
}

#[test]
fn search_bst_test() {
    let test_cases = vec![
        (vec![Some(4),Some(2),Some(7),Some(1),Some(3)], 2, vec![Some(2), Some(1), Some(3)]),
        (vec![Some(4),Some(2),Some(7),Some(1),Some(3)], 7, vec![Some(7)]),
        (vec![Some(4),Some(2),Some(7),Some(1),Some(3)], 9, vec![]),
    ];
    for case in test_cases {
        let tree = TreeNode::create_from_level_order(&case.0);
        assert_eq!(Solution::search_bst(tree, case.1).get_level_order_values(), case.2);
    }
}

#[test]
fn valid_ip_address_test() {
    let test_cases = vec![
        ("-172", "Neither"),
        ("-172.16", "Neither"),
        ("-172,16", "Neither"),
        ("-172.16.254.1", "Neither"),
        ("172.16.254.001", "Neither"),
        ("172:16:254:001", "Neither"),
        ("172.16.254.1", "IPv4"),
        ("172.16.254.0", "IPv4"),
        ("255.255.255.255", "IPv4"),
        ("2001:0db8:85a3:0000:0000:8a2e:0370:7334", "IPv6"),
        ("2001:db8:85a3:0:0:8A2E:0370:7334", "IPv6"),
        ("2001:0db8:85a3::8A2E:0370:7334", "Neither"),
        ("02001:0db8:85a3:0000:0000:8a2e:0370:7334", "Neither"),
        ("02001:0db8:85a3::0000:8a2e:0370:7334", "Neither"),
    ];
    for case in test_cases {
        assert_eq!(Solution::valid_ip_address(case.0.to_string()), case.1.to_string());
    }
}

#[test]
fn solve_test() {
    let test_cases = vec![
        (vec![vec!['O']], vec![vec!['O']]),
        (vec![vec!['X']], vec![vec!['X']]),
        (vec![vec!['O', 'O']], vec![vec!['O', 'O']]),
        (
            vec![
                vec!['O', 'O'],
                vec!['O', 'O']
            ],
             vec![
                 vec!['O', 'O'],
                 vec!['O', 'O']
             ],
        ),
        (
            vec![
                vec!['X', 'X', 'X'],
                vec!['X', 'X', 'X'],
                vec!['X', 'X', 'X'],
            ],
            vec![
                vec!['X', 'X', 'X'],
                vec!['X', 'X', 'X'],
                vec!['X', 'X', 'X'],
            ],
        ),
        (
            vec![
                vec!['X', 'X', 'X'],
                vec!['X', 'O', 'X'],
                vec!['X', 'X', 'X'],
            ],
            vec![
                vec!['X', 'X', 'X'],
                vec!['X', 'X', 'X'],
                vec!['X', 'X', 'X'],
            ],
        ),
        (
            vec![
                vec!['X', 'X', 'X'],
                vec!['X', 'O', 'X'],
                vec!['X', 'O', 'X'],
            ],
            vec![
                vec!['X', 'X', 'X'],
                vec!['X', 'O', 'X'],
                vec!['X', 'O', 'X'],
            ],
        ),
        (
            vec![
                vec!['X', 'X', 'X'],
                vec!['X', 'O', 'X'],
                vec!['X', 'X', 'O'],
            ],
            vec![
                vec!['X', 'X', 'X'],
                vec!['X', 'X', 'X'],
                vec!['X', 'X', 'O'],
            ],
        ),
        (
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'O', 'X'],
                vec!['X', 'X', 'O', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ],
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ],
        ),
        (
            vec![
                vec!['O','O','O','O','X','X'],
                vec!['O','O','O','O','O','O'],
                vec!['O','X','O','X','O','O'],
                vec!['O','X','O','O','X','O'],
                vec!['O','X','O','X','O','O'],
                vec!['O','X','O','O','O','O']
            ],
            vec![
                vec!['O','O','O','O','X','X'],
                vec!['O','O','O','O','O','O'],
                vec!['O','X','O','X','O','O'],
                vec!['O','X','O','O','X','O'],
                vec!['O','X','O','X','O','O'],
                vec!['O','X','O','O','O','O']
            ],
        ),
    ];
    for mut case in test_cases {
        Solution::solve(& mut case.0);
        assert_eq!(case.0, case.1);
    }
}

fn get_h_index_test_cases() -> Vec<(Vec<i32>, i32)>{
    vec![
        (vec![], 0),
        (vec![0], 0),
        (vec![1], 1),
        (vec![10], 1),
        (vec![10, 10], 2),
        (vec![0, 0, 0, 0], 0),
        (vec![1, 1, 1, 1], 1),
        (vec![2, 2, 2], 2),
        (vec![3, 3, 3], 3),
        (vec![1, 1, 1, 2], 1),
        (vec![0, 1, 3, 5, 6], 3),
        (vec![0, 4, 4, 5, 6], 4),
        (vec![0, 1, 10, 10, 12, 14], 4),
        (vec![0, 1, 1, 1, 12, 14], 2),
    ]
}

#[test]
fn h_index_ii_test() {
    for case in get_h_index_test_cases() {
        assert_eq!(Solution::h_index_ii(case.0), case.1);
    }
}

#[test]
fn h_index_ii_v2_test() {
    for case in get_h_index_test_cases() {
        assert_eq!(Solution::h_index_ii_v2(case.0), case.1);
    }
}