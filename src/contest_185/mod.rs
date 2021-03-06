#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::mem::swap;

struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        if s.is_empty() { return "".to_string(); }
        let mut alpha = Vec::with_capacity(s.len());
        let mut num = Vec::with_capacity(s.len());
        for c in s.chars() {
            if c.is_alphabetic() {
                alpha.push(c);
            }
            if c.is_numeric() {
                num.push(c);
            }
        }
        if (alpha.len() as i32 - num.len() as i32).abs() > 1 {
            "".to_string()
        } else {
            let mut res = Vec::with_capacity(s.len());
            if alpha.len() > num.len() {
                res.push(alpha.pop().unwrap());
                swap(&mut alpha, &mut num);
            } else if alpha.len() < num.len() {
                res.push(num.pop().unwrap());
            }
            for (a, n) in alpha.into_iter().zip(num.into_iter()) {
                res.push(a);
                res.push(n);
            }
            res.into_iter().collect::<String>()
        }
    }

    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut food: HashMap<String, HashMap<String, i32>> = HashMap::new();
        for o in orders {
            *food.entry(o[2].clone()).or_insert(HashMap::new()).entry(o[1].clone()).or_insert(0) += 1;
        }
        let mut food_names: Vec<String> = Vec::new();
        for name in food.keys() {
            food_names.push(name.clone());
        }
        food_names.sort();
        let mut table: HashMap<String, HashMap<String, i32>> = HashMap::new();
        for (name, qty_by_food_name) in food {
            for (tbl_num, qty) in qty_by_food_name {
                *table.entry(tbl_num).or_insert(HashMap::new()).entry(name.clone()).or_insert(0) += qty;
            }
        }

        let mut res: Vec<Vec<String>> = Vec::new();
        res.push(vec!["-1".to_string()]);
        for name in &food_names {
            res[0].push(name.clone());
        }
        for (tbl_num, qty_by_food_name) in table {
            let mut row = vec![tbl_num];
            for name in &food_names {
                if let Some(qty) = qty_by_food_name.get(name) {
                    row.push(qty.to_string());
                } else {
                    row.push("0".to_string());
                }
            }
            res.push(row);
        }
        res.sort_unstable_by(|a, b| a[0].parse::<i32>().unwrap().cmp(&b[0].parse::<i32>().unwrap()));
        res[0][0] = "Table".to_string();
        res
    }

    pub fn display_table_v2(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut fname_set: HashSet<String> = HashSet::new();
        let mut table: HashMap<String, HashMap<String, i32>> = HashMap::new();
        for ord in orders {
            let (tbl_num, fname) = (ord[1].clone(), ord[2].clone());
            fname_set.insert(fname.clone());
            *table.entry(tbl_num).or_insert(HashMap::new()).entry(fname).or_insert(0) += 1;
        }
        let mut fnames: Vec<String> = fname_set.into_iter().collect();
        fnames.sort();
        let mut res: Vec<Vec<String>> = Vec::new();
        res.push(vec!["Table".to_string()]);
        res[0].extend_from_slice(&fnames);
        let mut tbl_nums: Vec<String> = table.keys().map(|k| k.clone()).collect();
        tbl_nums.sort_unstable_by(|a, b| a.parse::<i32>().unwrap().cmp(&b.parse::<i32>().unwrap()));
        for tbl_num in tbl_nums {
            let mut row = vec![tbl_num.clone()];
            row.extend(fnames.iter().map(|fname| table[&tbl_num].get(fname).unwrap_or(&0).to_string()));
            res.push(row);
        }
        res
    }
}