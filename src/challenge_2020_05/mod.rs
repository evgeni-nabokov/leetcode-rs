mod trie;
mod trie_node;
mod list_node;
mod stock_spanner;

#[cfg(test)]
mod tests;

use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;
use std::cmp::{max, min, Ordering};

use crate::common::tree_node::TreeNode;

use list_node::ListNode;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        if jewels.is_empty() || stones.is_empty() { return 0; }
        let mut cnt = 0;
        let mut j_set: HashSet<char> = HashSet::with_capacity(jewels.len());
        for j in jewels.chars() {
            j_set.insert(j);
        }
        for s in stones.chars() {
            if j_set.contains(&s) {
                cnt += 1;
            }
        }
        cnt
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > 0 && magazine.len() == 0 { return false; }
        let mut avail_chars = HashMap::<char, usize>::with_capacity(magazine.len());
        for c in magazine.chars() {
            *avail_chars.entry(c).or_insert(0) += 1;
        }
        for c in ransom_note.chars() {
            if let Entry::Occupied(mut o) = avail_chars.entry(c) {
                if *o.get() > 0 {
                    *o.get_mut() -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn can_construct_v2(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > 0 && magazine.len() == 0 { return false; }
        let mut chars = vec![0; 26];
        for c in magazine.chars() {
            chars[c as usize - 97] += 1;
        }
        for c in ransom_note.chars() {
            let i = c as usize - 97;
            if chars[i] == 0 {
                return false;
            } else {
                chars[i] -= 1;
            }
        }
        true
    }

    pub fn bitwise_complement(n: i32) -> i32 {
        let mut res = 0;
        let mut un = n as u32;
        let mut i = 0;

        loop {
            if un & 1 == 0 {
                res += 1 << i;
            }
            i += 1;
            un >>= 1;
            if un == 0 {
                break;
            }
        }
        res
    }

    pub fn bitwise_complement_v2(n: i32) -> i32 {
        max((n as u32 + 1).next_power_of_two() as i32, 2) - n - 1
    }

    pub fn first_uniq_char(s: String) -> i32 {
        if s.is_empty() { return -1; }
        let mut chars = vec![0; 26];
        for c in s.chars() {
            chars[c as usize - 97] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if chars[c as usize - 97] == 1 {
                return i as i32;
            }
        }
        -1
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counter_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len() / 2);
        for &n in nums.iter() {
            match counter_map.entry(n) {
                Entry::Occupied(o) if *o.get() >= nums.len() / 2 => {
                    return n;
                },
                Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                },
                Entry::Vacant(v) => {
                    v.insert(1);
                }
            }
        }
        *counter_map.keys().next().unwrap()
    }

    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        // 1 or 2 points are always belong to the same line.
        if coordinates.len() < 3 { return false; }
        // a * x + b * y + c = 0.
        // a = y1 - y2, b = x2 - x1, c = x1 * y2 - x2 * y1.
        let (x_1, y_1) = (coordinates[0][0], coordinates[0][1]);
        let (x_2, y_2) = (coordinates[1][0], coordinates[1][1]);
        let a = y_1 - y_2;
        let b = x_2 - x_1;
        let c = x_1 * y_2 - x_2 * y_1;
        // Check each point if it satisfies the equation of line above.
        for point in coordinates.iter().skip(2) {
            let (x, y) = (point[0], point[1]);
            if a * x + b * y + c != 0 {
                return false;
            }
        }
        true
    }

    pub fn is_perfect_square(num: i32) -> bool {
        let mut sum = 0;
        let mut odd_n = 1;
        loop {
            if sum == num - odd_n {
                return true;
            }
            if sum > num - odd_n {
                return false;
            }
            sum += odd_n;
            odd_n += 2;
        }
    }

    pub fn is_perfect_square_v2(num: i32) -> bool {
        if num == 1 { return true; }
        let mut x_prev = 1f64;
        let mut x = (num / 2) as f64;
        while (x_prev - x).abs() >= 1.0 {
            x_prev = x;
            x = (x + num as f64 / x) / 2.0;
        }
        let mut possible_roots = vec![x_prev.floor() as i32, x.floor() as i32];
        possible_roots.sort();
        possible_roots.dedup();
        for r in possible_roots {
            if r * r == num {
                return true
            }
        }
        false
    }

    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut who_trusts = vec![0; n as usize];
        let mut who_is_trusted = vec![0; n as usize];
        for t in trust.iter() {
            who_trusts[(t[0] - 1) as usize] += 1;
            who_is_trusted[(t[1] - 1) as usize] += 1;
        }
        let mut judge = -1i32;
        for i in 0..n as usize {
            if who_trusts[i] == 0 {
                if judge > 0 {
                    return -1;
                } else if who_is_trusted[i] == n - 1 {
                    judge = i as i32 + 1;
                } else {
                    return -1;
                }
            }
        }
        judge
    }

    pub fn find_judge_v2(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut counter = vec![0; n as usize];
        for t in trust.iter() {
            counter[(t[0] - 1) as usize] -= 1;
            counter[(t[1] - 1) as usize] += 1;
        }
        match counter.iter().position(|&p| p == n - 1) {
            Some(i) => i as i32 + 1,
            _ => -1,
        }
    }

    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        fn fill(image: &mut Vec<Vec<i32>>, r: usize, c: usize, new_color: i32, old_color: i32) {
            if image[r][c] != old_color || image[r][c] == new_color {
                return;
            }
            image[r][c] = new_color;
            if r > 0 {
                fill(image, r - 1, c, new_color, old_color);
            }
            if r < image.len() - 1 {
                fill(image, r + 1, c, new_color, old_color);
            }
            if c > 0 {
                fill(image, r, c - 1, new_color, old_color);
            }
            if c < image[0].len() - 1 {
                fill(image, r, c + 1, new_color, old_color);
            }
        }
        let old_color = image[sr as usize][sc as usize];
        fill(&mut image, sr as usize, sc as usize, new_color, old_color);
        image
    }

    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let mid = left + ((right - left) / 2);
            if mid == 0 ||
                nums[mid] != nums[mid - 1] && nums[mid] != nums[mid + 1] {
                return nums[mid];
            }
            let start = if nums[mid] == nums[mid + 1] { mid } else { mid - 1 };
            if start % 2 == 0 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        nums[left]
    }

    pub fn remove_k_digits(num: String, mut k: i32) -> String {
        if num.len() == k as usize { return "0".to_string(); }
        let mut res: Vec<char> = Vec::with_capacity(k as usize);
        let digits: Vec<char> = num.chars().collect();
        for d in digits.iter() {
            while k > 0 && !res.is_empty() && res.last().unwrap() > d {
                res.pop();
                k -= 1;
            }
            if !res.is_empty() || *d != '0' {
                res.push(*d);
            }
        }
        while k > 0 {
            res.pop();
            k -= 1;
        }
        while !res.is_empty() && res[0] == '0' {
            res.remove(0);
        }
        if res.is_empty() { "0".to_string() } else { res.iter().collect() }
    }

    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        let mut sum: i32 = nums[0];
        let mut sum_inv: i32 = -nums[0];
        let mut max_sum: i32 = sum;
        let mut max_sum_inv: i32 = sum_inv;
        let mut total_sum_inv = sum_inv;
        for &n in nums.iter().skip(1) {
            sum = max(n, sum + n);
            sum_inv = max(-n, sum_inv - n);
            max_sum = max(sum, max_sum);
            max_sum_inv = max(sum_inv, max_sum_inv);
            total_sum_inv -= n;
        }
        if max_sum < 0 {
            max_sum
        } else {
            max(max_sum, max_sum_inv - total_sum_inv)
        }
    }

    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }
        let mut odd_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(head.as_ref().unwrap().val)));
        if head.as_ref().unwrap().next.is_none() {
            return odd_head;
        }
        let mut even_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(head.as_ref().unwrap().next.as_ref().unwrap().val)));
        let mut curr_odd_node: &mut Option<Box<ListNode>> = &mut odd_head;
        let mut curr_even_node: &mut Option<Box<ListNode>> = &mut even_head;
        let mut curr_node: &Option<Box<ListNode>> = &head.as_ref().unwrap().next.as_ref().unwrap().next;
        let mut pos = 1;
        while curr_node.is_some() {
            let new_node = Some(Box::new(ListNode::new(curr_node.as_ref().unwrap().val)));
            if pos % 2 == 0 {
                curr_even_node.as_mut().unwrap().next = new_node;
                curr_even_node = &mut curr_even_node.as_mut().unwrap().next;
            } else {
                curr_odd_node.as_mut().unwrap().next = new_node;
                curr_odd_node = &mut curr_odd_node.as_mut().unwrap().next;
            }
            curr_node = &curr_node.as_ref().unwrap().next;
            pos += 1;
        }
        curr_odd_node.as_mut().unwrap().next = even_head;
        odd_head
    }

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() || s.is_empty() || p.is_empty() { return vec![]; }
        let mut res: Vec<i32> = Vec::new();
        let schars: Vec<char> = s.chars().collect();
        let mut pchars_counter = vec![0; 26];
        for c in p.chars() {
            pchars_counter[c as usize - 97] += 1;
        }
        for start in 0..=(s.len() - p.len()) {
            let mut counter = pchars_counter.clone();
            for i in 0..p.len() {
                let j = schars[start + i] as usize - 97;
                if counter[j] == 0 {
                    break;
                } else {
                    counter[j] -= 1;
                }
                if i == p.len() - 1 {
                    res.push(start as i32);
                }
            }
        }
        res
    }

    pub fn find_anagrams_v2(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() || s.is_empty() || p.is_empty() { return vec![]; }
        const BASE: usize = 'a' as usize;
        let mut res: Vec<i32> = Vec::new();
        let schars: Vec<char> = s.chars().collect();
        let mut pchars_counter = vec![0; 26];
        let mut schars_counter = vec![0; 26];
        for i in 0..p.len() {
            schars_counter[schars[i] as usize - BASE] += 1;
        }
        for c in p.chars() {
            pchars_counter[c as usize - BASE] += 1;
        }
        if schars_counter == pchars_counter {
            res.push(0);
        }
        for i in 1..=(s.len() - p.len()) {
            schars_counter[schars[i - 1] as usize - BASE] -= 1;
            schars_counter[schars[i + p.len() - 1] as usize - BASE] += 1;
            if schars_counter == pchars_counter {
                res.push(i as i32);
            }
        }
        res
    }

    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s2.len() < s1.len() || s1.is_empty() || s2.is_empty() { return false; }
        const BASE: usize = 'a' as usize;
        let s2_chars: Vec<char> = s2.chars().collect();
        let mut s1_counter = vec![0; 26];
        let mut s2_counter = vec![0; 26];
        for c in s1.chars() {
            s1_counter[c as usize - BASE] += 1;
        }
        for i in 0..s1.len() {
            s2_counter[s2_chars[i] as usize - BASE] += 1;
        }
        if s1_counter == s2_counter {
            return true;
        }
        for i in 1..=(s2.len() - s1.len()) {
            s2_counter[s2_chars[i - 1] as usize - BASE] -= 1;
            s2_counter[s2_chars[i + s1.len() - 1] as usize - BASE] += 1;
            if s1_counter == s2_counter {
                return true;
            }
        }
        false
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        let mut node = root.clone();
        let mut cnt = k.clone();
        loop {
            while node.is_some() {
                stack.push(node.clone());
                node = RefCell::borrow(node.clone().as_ref().unwrap()).left.clone()
            }
            node = stack.pop().unwrap();
            cnt -= 1;
            if cnt == 0 {
                return node.as_ref().unwrap().borrow().val;
            }
            node = RefCell::borrow(node.clone().as_ref().unwrap()).right.clone()
        }
    }

    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() { return 0; }
        let n_rows = matrix.len();
        let n_cols = matrix[0].len();
        let mut cnt = 0;
        for r in 0..n_rows {
            for c in 0..n_cols {
                if matrix[r][c] == 1 {
                    cnt += 1;
                    // Check all submatrices growing from to the right-top.
                    let max_size = min(n_rows - r, n_cols - c);
                    'outer: for s in 1..max_size {
                        for rr in r..r + s {
                            if matrix[rr][c + s] == 0 {
                                break 'outer;
                            }
                        }
                        for cc in c..c + s {
                            if matrix[r + s][cc] == 0 {
                                break 'outer;
                            }
                        }
                        if matrix[r + s][c + s] == 0 {
                            break 'outer;
                        }
                        cnt += 1;
                    }
                }
            }
        }
        cnt
    }

    pub fn frequency_sort(s: String) -> String {
        let mut cnt: Vec<(char, usize)> = vec![
            ('A', 0), ('B', 0), ('C', 0), ('D', 0), ('E', 0), ('F', 0), ('G', 0), ('H', 0), ('I', 0),
            ('J', 0), ('K', 0), ('L', 0), ('M', 0), ('N', 0), ('O', 0), ('P', 0), ('Q', 0), ('R', 0),
            ('S', 0), ('T', 0), ('U', 0), ('V', 0), ('W', 0), ('X', 0), ('Y', 0), ('Z', 0),
            ('a', 0), ('b', 0), ('c', 0), ('d', 0), ('e', 0), ('f', 0), ('g', 0), ('h', 0), ('i', 0),
            ('j', 0), ('k', 0), ('l', 0), ('m', 0), ('n', 0), ('o', 0), ('p', 0), ('q', 0), ('r', 0),
            ('s', 0), ('t', 0), ('u', 0), ('v', 0), ('w', 0), ('x', 0), ('y', 0), ('z', 0),
        ];
        for c in s.chars() {
            let ascii_index = c as u8;
            let i = ascii_index - if ascii_index < 'Z' as u8 { 'A' as u8 } else { 'a' as u8 - 26 };
            cnt[i as usize].1 += 1;
        }
        cnt.sort_unstable_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => a.0.cmp(&b.0),
            o => o,
        });
        let mut sorted_chars: Vec<char> = Vec::with_capacity(s.len());
        for e in cnt.iter() {
            sorted_chars.extend_from_slice(&vec![e.0; e.1])
        }
        sorted_chars.iter().collect()
    }

    pub fn frequency_sort_v2(s: String) -> String {
        let mut map: HashMap<char, usize> = HashMap::with_capacity(s.len());
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut chars: Vec<(char, usize)> = map.iter().map(|(&k, &v)| (k, v)).collect();
        chars.sort_unstable_by(|a, b| match b.1.cmp(&a.1) {
            Ordering::Equal => a.0.cmp(&b.0),
            o => o,
        });
        let mut sorted_chars: Vec<char> = Vec::with_capacity(s.len());
        for e in chars.iter() {
            sorted_chars.extend_from_slice(&vec![e.0; e.1])
        }
        sorted_chars.iter().collect()
    }

    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if a.is_empty() || b.is_empty() { return vec![]; }
        let mut a_idx = 0;
        let mut b_idx = 0;
        let mut res: Vec<Vec<i32>> = Vec::new();
        while a_idx < a.len() && b_idx < b.len() {
            let a_int = a[a_idx].clone();
            let b_int = b[b_idx].clone();
            if a_int[1] >= b_int[0] && a_int[0] <= b_int[1] {
                res.push(vec![max(a_int[0], b_int[0]), min(a_int[1], b_int[1])]);
            }
            if a_int[1] > b_int[1] {
                b_idx += 1;
            } else {
                a_idx += 1;
            }
        }
        res
    }

    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_bst(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if arr.is_empty() { return None; }
            let val = arr[0];
            let root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            if arr.len() == 1 { return root; }
            let mut i = 1;
            while i < arr.len() && arr[i] < val { i += 1; };
            RefCell::borrow_mut(root.as_ref().unwrap()).left = build_bst(&arr[1..i]);
            RefCell::borrow_mut(root.as_ref().unwrap()).right = build_bst(&arr[i..]);
            root
        }
        build_bst(&preorder)
    }

    pub fn bst_from_preorder_v2(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_bst(arr: &[i32], mut i: usize, mut curr_node: RefMut<TreeNode>, left: Option<i32>, right: Option<i32>) -> usize {
            if i == arr.len()
                || (left.is_some() && arr[i] < *left.as_ref().unwrap())
                || (right.is_some() && arr[i] > *right.as_ref().unwrap()) {
                return i;
            }

            if arr[i] < curr_node.val {
                curr_node.left = Some(Rc::new(RefCell::new(TreeNode::new(arr[i]))));
                //let aa = RefCell::borrow_mut(curr_node.left.as_ref().unwrap());
                i = build_bst(arr, i + 1, RefCell::borrow_mut(curr_node.left.as_ref().unwrap()), left, Some(curr_node.val - 1));
                if i == arr.len()
                    || (left.is_some() && arr[i] < *left.as_ref().unwrap())
                    || (right.is_some() && arr[i] > *right.as_ref().unwrap()) {
                    return i;
                }
            }
            curr_node.right = Some(Rc::new(RefCell::new(TreeNode::new(arr[i]))));
            i = build_bst(arr, i + 1, RefCell::borrow_mut(curr_node.right.as_ref().unwrap()), Some(curr_node.val + 1), right);
            return i;
        }
        if preorder.is_empty() { return None; }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        if preorder.len() == 1 { return root; }
        build_bst(&preorder, 1, RefCell::borrow_mut(root.as_ref().unwrap()), None, None);
        root
    }

    pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
        if a.is_empty() || b.is_empty() { return 0; }
        let mut table: Vec<Vec<i32>> = vec![vec![0; b.len() + 1]; a.len() + 1];
        for row in 1..=a.len() {
            for col in 1..=b.len() {
                if a[row - 1] == b[col - 1] {
                    table[row][col] = 1 + table[row - 1][col - 1];
                } else {
                    table[row][col] = max(table[row - 1][col], table[row][col - 1]);
                }
            }
        }
        table[a.len()][b.len()]
    }

    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        if n < 2 || dislikes.is_empty() { return true; }
        // Building adjacency list of the given graph.
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n as usize];
        for pair in dislikes.iter() {
            let i_1 = (pair[0] - 1) as usize;
            let i_2 = (pair[1] - 1) as usize;
            adj_list[i_1].push(i_2);
            adj_list[i_2].push(i_1);
        }
        // Coloring the graph with two colors: -1 and 1.
        // v keeps color of each vertex. An uncolored vertex has the color of 0.
        let mut v: Vec<i8> = vec![0; n as usize];
        // Using deep first search.
        fn dfs(i: usize, color: i8, adj_list: &Vec<Vec<usize>>, v: &mut Vec<i8>) -> bool {
            v[i] = color;
            for &k in adj_list[i].iter() {
                if v[k] == v[i] || v[k] == 0 && !dfs(k, -color, adj_list, v) {
                    return false;
                }
            }
            true
        }
        // Applying dfs for each uncolored vertex.
        for i in 0..n as usize {
            if v[i] == 0 && !dfs(i, 1, &adj_list, &mut v){
                return false;
            }
        }
        true
    }

    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; (num + 1) as usize];
        let mut j = 0usize;
        for i in 1..=num as usize {
            if i & (i - 1) == 0 {
                j = 0;
            }
            res[i] = 1 + res[j];
            j += 1;
        }
        res
    }

    pub fn count_bits_v2(num: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; (num + 1) as usize];
        for i in 1..=num {
            res[i as usize] = 1 + res[(i - (i & -i)) as usize];
        }
        res
    }

    pub fn count_bits_v3(num: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; (num + 1) as usize];
        for i in 1..=num {
            res[i as usize] = 1 + res[(i & (i - 1)) as usize];
        }
        res
    }

    pub fn can_finish(n: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if n < 2 || prerequisites.is_empty() { return true; }
        // Building adjacency list of the given graph.
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n as usize];
        for pair in prerequisites.iter() {
            adj_list[pair[1] as usize].push(pair[0] as usize);
        }
        // 0 - not visited,
        // 1 - visited, but the traversal is not finished yet,
        // 2 - visited, traversal is finished.
        let mut visited: Vec<i8> = vec![0; n as usize];
        // Using deep first search.
        fn dfs(i: usize, adj_list: &Vec<Vec<usize>>, visited: &mut Vec<i8>) -> bool {
            if visited[i] == 1 { return false; }
            if visited[i] == 2 { return true; }
            visited[i] = 1;
            for &k in adj_list[i].iter() {
                if !dfs(k, adj_list, visited) {
                    return false;
                }
            }
            visited[i] = 2;
            true
        }

        for i in 0..n as usize {
            if visited[i] == 0 && !dfs(i, &adj_list, &mut visited){
                return false;
            }
        }
        true
    }
}
