use super::*;

#[test]
fn string_matching_test() {
    assert_eq!(Solution::reformat("a0b1c2".to_string()), "a0b1c2");
    assert_eq!(Solution::reformat("covid2019".to_string()), "d2c0o1v9i");
    assert_eq!(Solution::reformat("1229857369".to_string()), "");
}

#[test]
fn display_table_test() {
    let input = vec![
        vec!["David".to_string(),"3".to_string(),"Ceviche".to_string()],
        vec!["Corina".to_string(),"10".to_string(),"Beef Burrito".to_string()],
        vec!["David".to_string(),"3".to_string(),"Fried Chicken".to_string()],
        vec!["Carla".to_string(),"5".to_string(),"Water".to_string()],
        vec!["Carla".to_string(),"5".to_string(),"Ceviche".to_string()],
        vec!["Rous".to_string(),"3".to_string(),"Ceviche".to_string()],
    ];

    let output = vec![
        vec!["Table".to_string(),"Beef Burrito".to_string(),"Ceviche".to_string(),"Fried Chicken".to_string(),"Water".to_string()],
        vec!["3".to_string(),"0".to_string(),"2".to_string(),"1".to_string(),"0".to_string()],
        vec!["5".to_string(),"0".to_string(),"1".to_string(),"0".to_string(),"1".to_string()],
        vec!["10".to_string(),"1".to_string(),"0".to_string(),"0".to_string(),"0".to_string()],
    ];

    assert_eq!(Solution::display_table(input), output);
}