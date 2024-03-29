#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter::FromIterator;
use std::mem::swap;
use std::rc::Rc;

use lazy_static::lazy_static;

use crate::common::list_node::ListNode;
use crate::common::tree_node::TreeNode;

lazy_static! {
    static ref UGLY_NUMBERS: Vec<i32> = {
        let mut n = 1690;
        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;
        let mut nums: Vec<i32> = Vec::with_capacity(n);
        nums.push(1);
        n -= 1;
        while n > 0 {
            let u = (nums[i5] * 5).min((nums[i2] * 2).min(nums[i3] * 3));
            if u == nums[i2] * 2 {
                i2 += 1;
            }
            if u == nums[i3] * 3 {
                i3 += 1;
            }
            if u == nums[i5] * 5 {
                i5 += 1;
            }
            nums.push(u);
            n -= 1;
        }
        nums
    };
}

struct Solution {}

impl Solution {
    // 441. Arranging Coins.
    // https://leetcode.com/problems/arranging-coins/
    pub fn arrange_coins(n: i32) -> i32 {
        (((8f64 * n as f64 + 1f64).sqrt() - 1f64) / 2f64).floor() as i32
    }

    // 107. Binary Tree Level Order Traversal II.
    // https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<Vec<i32>>, level: usize) {
            if node.is_none() {
                return;
            }
            if level == levels.len() {
                levels.push(vec![]);
            }
            levels[level].push(RefCell::borrow(node.as_ref().unwrap()).val);
            dfs(
                RefCell::borrow(node.as_ref().unwrap()).left.clone(),
                levels,
                level + 1,
            );
            dfs(
                RefCell::borrow(node.as_ref().unwrap()).right.clone(),
                levels,
                level + 1,
            );
        }

        let mut levels: Vec<Vec<i32>> = Vec::new();
        dfs(root, &mut levels, 0);
        levels.into_iter().rev().collect()
    }

    // 957. Prison Cells After N Days.
    // https://leetcode.com/problems/prison-cells-after-n-days/
    pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
        if cells.is_empty() {
            return vec![];
        }
        if n == 0 {
            return cells;
        }
        let l = cells.len();
        let mut state_vec: Vec<Vec<i32>> = Vec::new();
        let mut state_map: HashMap<Vec<i32>, usize> = HashMap::new();
        let mut i = 0usize;
        let mut prev_c = cells[0];
        loop {
            for c in 1..l - 1 {
                if prev_c + cells[c + 1] == 1 {
                    prev_c = cells[c];
                    cells[c] = 0;
                } else {
                    prev_c = cells[c];
                    cells[c] = 1;
                }
            }
            cells[0] = 0;
            cells[l - 1] = 0;
            prev_c = 0;
            i += 1;
            if i == n as usize {
                return cells;
            }
            if let Some(start) = state_map.get(&cells) {
                let j = (n as usize - *start) % (i - *start);
                if j == 0 {
                    return cells;
                }
                return state_vec[j].clone();
            }
            state_vec.push(cells.clone());
            state_map.insert(cells.clone(), i);
        }
    }

    pub fn prison_after_n_days_v2(mut cells: Vec<i32>, mut n: i32) -> Vec<i32> {
        if cells.is_empty() {
            return vec![];
        }
        if n == 0 {
            return cells;
        }
        let l = cells.len();
        let mut state_map: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut prev_c = cells[0];
        let mut is_fast_forwarded = false;
        while n > 0 {
            if !is_fast_forwarded {
                if let Some(start) = state_map.get(&cells) {
                    n %= *start as i32 - n;
                    is_fast_forwarded = true;
                } else {
                    state_map.insert(cells.clone(), n);
                }
            }
            if n > 0 {
                for c in 1..l - 1 {
                    if prev_c + cells[c + 1] == 1 {
                        prev_c = cells[c];
                        cells[c] = 0;
                    } else {
                        prev_c = cells[c];
                        cells[c] = 1;
                    }
                }
                cells[0] = 0;
                cells[l - 1] = 0;
                prev_c = 0;
                n -= 1;
            }
        }
        cells
    }

    // 264. Ugly Number II.
    // https://leetcode.com/problems/ugly-number-ii/
    pub fn nth_ugly_number(n: i32) -> i32 {
        UGLY_NUMBERS[n as usize - 1]
    }

    // 461. Hamming Distance.
    // https://leetcode.com/problems/hamming-distance/
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut n = 1;
        let mut cnt = 0;
        for _ in 1..32 {
            if x & n != y & n {
                cnt += 1;
            }
            n <<= 1;
        }
        cnt
    }

    pub fn hamming_distance_v2(x: i32, y: i32) -> i32 {
        let mut z = x ^ y;
        let mut distance = 0;
        while z != 0 {
            distance += 1;
            z = z & (z - 1);
        }
        distance
    }

    pub fn hamming_distance_v3(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }

    // 66. Plus One.
    // https://leetcode.com/problems/plus-one/
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut i: isize = digits.len() as isize - 1;
        while i >= 0 {
            let ui = i as usize;
            if digits[ui] == 9 {
                digits[ui] = 0;
                i -= 1
            } else {
                digits[ui] += 1;
                break;
            }
        }
        if i < 0 {
            digits.push(0);
            digits[0] = 1;
        }
        digits
    }

    // 463. Island Perimeter.
    // https://leetcode.com/problems/island-perimeter/
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let mut p = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let c = grid[row][col];
                if c == 0 {
                    continue;
                }
                if row == 0 || grid[row - 1][col] == 0 {
                    p += 1;
                }
                if col == 0 || grid[row][col - 1] == 0 {
                    p += 1;
                }
                if row == grid.len() - 1 || grid[row + 1][col] == 0 {
                    p += 1;
                }
                if col == grid[0].len() - 1 || grid[row][col + 1] == 0 {
                    p += 1;
                }
            }
        }
        p
    }

    // 15. 3Sum.
    // https://leetcode.com/problems/3sum/
    // Two pointers method.
    // Time complexity: O(N^2).
    // Space complexity: O(1).
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if nums.len() < 3 {
            return res;
        }

        nums.sort_unstable();

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                match sum.cmp(&0) {
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                    }
                    Ordering::Greater => {
                        right -= 1;
                    }
                    Ordering::Less => {
                        left += 1;
                    }
                }
            }
        }
        res
    }

    // Binary search method.
    // Time complexity: O(N^2 * LogN).
    // Space complexity: O(1).
    pub fn three_sum_v2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if nums.len() < 3 {
            return res;
        }

        nums.sort_unstable();

        //for i in 0..nums.len() - 2 {
        let mut i = 0;
        while i < nums.len() - 2 && nums[i] <= 0 {
            if i > 0 && nums[i] == nums[i - 1] {
                i += 1;
                continue;
            }

            for j in (i + 1)..nums.len() - 1 {
                if j > (i + 1) && nums[j] == nums[j - 1] {
                    continue;
                }

                let complement = -nums[i] - nums[j];
                if let Ok(k) = nums[j + 1..].binary_search(&complement) {
                    res.push(vec![nums[i], nums[j], nums[j + 1 + k]]);
                }
            }

            i += 1;
        }

        res
    }

    // One-pass hash map method.
    // Time complexity: O(N^2).
    // Space complexity: O(N).
    pub fn three_sum_v3(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if nums.len() < 3 {
            return res;
        }

        nums.sort_unstable();

        let mut i = 0;
        while i < nums.len() - 2 && nums[i] <= 0 {
            if i > 0 && nums[i] == nums[i - 1] || nums[i] > 0 {
                i += 1;
                continue;
            }

            let mut j = i + 1;
            let mut set = HashSet::new();

            while j < nums.len() {
                let complement = -nums[i] - nums[j];
                if set.contains(&complement) {
                    res.push(vec![nums[i], nums[j], complement]);
                    while j + 1 < nums.len() && nums[j] == nums[j + 1] {
                        j += 1;
                    }
                }
                set.insert(nums[j]);
                j += 1;
            }

            i += 1;
        }

        res
    }

    // 662. Maximum Width of Binary Tree.
    // https://leetcode.com/problems/maximum-width-of-binary-tree/
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut max_width = 0u32;
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, u32)> = VecDeque::new();
        queue.push_back((root.unwrap(), 0));
        while !queue.is_empty() {
            let level_head_index = queue[0].1;
            let mut col_index = 0u32;
            let lev_len = queue.len();
            for _ in 0..lev_len {
                let (node, i) = queue.pop_front().unwrap();
                col_index = i;
                if RefCell::borrow(&node).left.is_some() {
                    queue.push_back((RefCell::borrow(&node).left.clone().unwrap(), 2 * col_index));
                }
                if RefCell::borrow(&node).right.is_some() {
                    queue.push_back((
                        RefCell::borrow(&node).right.clone().unwrap(),
                        2 * col_index + 1,
                    ));
                }
            }
            max_width = max_width.max(col_index - level_head_index + 1);
        }
        max_width as i32
    }

    // 78. Subsets.
    // https://leetcode.com/problems/subsets/
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![nums];
        }
        if nums.len() == 1 {
            return vec![vec![], nums];
        }
        let max_len = nums.len();
        let mut res: Vec<Vec<i32>> = Vec::new();
        res.push(Vec::new());
        for l in 1..=nums.len() {
            let mut idx: Vec<usize> = vec![0; l];
            for i in 1..l {
                idx[i] = i;
            }
            let mut max: Vec<usize> = vec![0; l];
            for i in 0..l {
                max[i] = i + max_len - l;
            }
            loop {
                let mut set: Vec<i32> = Vec::with_capacity(l);
                for i in 0..l {
                    set.push(nums[idx[i]]);
                }
                res.push(set);
                let mut i = l as isize - 1;
                while i >= 0 && idx[i as usize] == max[i as usize] {
                    i -= 1;
                }
                if i < 0 {
                    break;
                }
                idx[i as usize] += 1;
                let mut k = idx[i as usize] + 1;
                for j in i as usize + 1..l {
                    idx[j] = k;
                    k += 1;
                }
            }
        }
        res
    }

    // 100. Same Tree.
    // https://leetcode.com/problems/same-tree/
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn solve(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(pref), Some(qref)) => {
                    let pnode = RefCell::borrow(pref);
                    let qnode = RefCell::borrow(qref);
                    pnode.val == qnode.val
                        && solve(&pnode.left, &qnode.left)
                        && solve(&pnode.right, &qnode.right)
                }
                _ => false,
            }
        }
        solve(&p, &q)
    }

    pub fn is_same_tree_v2(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        p == q
    }

    // 1344. Angle Between Hands of a Clock.
    // https://leetcode.com/problems/angle-between-hands-of-a-clock/
    pub fn angle_clock(hours: i32, minutes: i32) -> f64 {
        let h = if hours == 12 { 0f64 } else { hours as f64 };
        let m = minutes as f64;
        let a = (m * 6f64 - (h + m / 60f64) * 30f64).abs();
        if a > 180f64 {
            360f64 - a
        } else {
            a
        }
    }

    // 151. Reverse Words in a String.
    // https://leetcode.com/problems/reverse-words-in-a-string/
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }

    // 50. Pow(x, n).
    // https://leetcode.com/problems/powx-n/
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1f64;
        }
        let mut p: i64 = n as i64;
        if p < 0 {
            x = 1f64 / x;
            p = -p;
        }
        let mut res = 1f64;
        let mut curr_prod = x;
        let mut i = p;
        while i > 0 {
            if i % 2 == 1 {
                res *= curr_prod;
            }
            curr_prod *= curr_prod;
            i /= 2;
        }
        res
    }

    // 347. Top K Frequent Elements.
    // https://leetcode.com/problems/top-k-frequent-elements/
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct Entry {
            number: i32,
            count: usize,
        }

        // Manually implement Ord so we get a min-heap instead of a max-heap.
        impl Ord for Entry {
            fn cmp(&self, other: &Self) -> Ordering {
                other.count.cmp(&self.count)
            }
        }

        impl PartialOrd for Entry {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        if nums.len() == k as usize {
            return nums;
        }

        let mut counter: HashMap<i32, usize> = HashMap::new();
        for n in nums {
            *counter.entry(n).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<Entry> = BinaryHeap::with_capacity(counter.len());
        for (number, count) in counter.into_iter() {
            heap.push(Entry { number, count });
        }
        heap.into_sorted_vec()
            .into_iter()
            .take(k as usize)
            .map(|x| x.number)
            .collect()
    }

    // 210. Course Schedule II.
    // https://leetcode.com/problems/course-schedule-ii/
    pub fn find_order(n: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![0];
        }

        // Building adjacency list of the given graph.
        let mut adj_list: Vec<Vec<i32>> = vec![vec![]; n as usize];
        for pair in prerequisites {
            adj_list[pair[0] as usize].push(pair[1]);
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        enum Color {
            White, // Not visited.
            Gray,  // Visited, but the traversal is not finished yet.
            Black, // Visited, traversal is finished.
        }

        // Using deep first search.
        fn dfs(
            v: i32,
            adj_list: &Vec<Vec<i32>>,
            visited: &mut Vec<Color>,
            order: &mut Vec<i32>,
            mut c: i32,
        ) -> i32 {
            let i = v as usize;
            visited[i] = Color::Gray;
            for &k in &adj_list[i] {
                if visited[k as usize] == Color::Gray {
                    return -1;
                }
                if visited[k as usize] == Color::White {
                    c = dfs(k, adj_list, visited, order, c);
                    if c == -1 {
                        return -1;
                    }
                }
            }
            visited[i] = Color::Black;
            order[c as usize] = v;
            c + 1
        }

        let mut visited: Vec<Color> = vec![Color::White; n as usize];
        let mut order: Vec<i32> = vec![0; n as usize];
        let mut c = 0;
        for v in 0..n {
            if visited[v as usize] == Color::White {
                c = dfs(v, &adj_list, &mut visited, &mut order, c);
                if c == -1 {
                    return vec![];
                }
            }
        }
        order
    }

    // 67. Add Binary.
    // https://leetcode.com/problems/add-binary/
    pub fn add_binary(a: String, b: String) -> String {
        let mut ac: Vec<char> = a.chars().collect();
        let mut bc: Vec<char> = b.chars().collect();
        if ac.len() > bc.len() {
            swap(&mut ac, &mut bc);
        }
        let mut ai = ac.len() - 1;
        let mut bi = bc.len() - 1;
        let mut carry = false;
        loop {
            match (ac[ai], bc[bi], carry) {
                ('1', '1', true) => {
                    bc[bi] = '1';
                    carry = true;
                }
                ('0', '0', false) => {
                    bc[bi] = '0';
                    carry = false;
                }
                ('1', '1', false) | ('1', '0', true) | ('0', '1', true) => {
                    bc[bi] = '0';
                    carry = true;
                }
                ('0', '0', true) | ('1', '0', false) | ('0', '1', false) => {
                    bc[bi] = '1';
                    carry = false;
                }
                _ => unreachable!(),
            }
            if ai == 0 {
                break;
            }
            ai -= 1;
            bi -= 1;
        }

        if carry && bi != 0 {
            bi -= 1;
            loop {
                if bc[bi] == '0' {
                    bc[bi] = '1';
                    carry = false;
                    break;
                } else {
                    bc[bi] = '0';
                }
                if bi == 0 {
                    break;
                }
                bi -= 1;
            }
        }

        if carry {
            vec!['1'].into_iter().chain(bc.into_iter()).collect()
        } else {
            bc.into_iter().collect()
        }
    }

    // 203. Remove Linked List Elements.
    // https://leetcode.com/problems/remove-linked-list-elements/
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut fake_head = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut prev_node: &mut Box<ListNode> = &mut fake_head;
        while let Some(some) = prev_node.next.as_ref() {
            if some.val == val {
                prev_node.next = prev_node.next.as_mut().unwrap().next.take();
            } else {
                prev_node = prev_node.next.as_mut().unwrap();
            }
        }
        fake_head.next
    }

    // 79. Word Search.
    // https://leetcode.com/problems/word-search/
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(board: &mut Vec<Vec<char>>, word: &[u8], i: usize, r: isize, c: isize) -> bool {
            if i >= word.len() {
                return false;
            }
            if r < 0 || c < 0 || r as usize >= board.len() || c as usize >= board[0].len() {
                return false;
            }

            let ur = r as usize;
            let uc = c as usize;
            let ch = board[ur][uc];
            if word[i] != ch as u8 {
                return false;
            }
            if i == word.len() - 1 {
                return true;
            }

            board[ur][uc] = '#';
            for (x, y) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
                if dfs(board, word, i + 1, r + y, c + x) {
                    return true;
                }
            }
            board[ur][uc] = ch;
            false
        }

        let bytes = word.as_bytes();
        for r in 0..board.len() as isize {
            for c in 0..board[0].len() as isize {
                if dfs(&mut board, bytes, 0, r, c) {
                    return true;
                }
            }
        }
        false
    }

    // 103. Binary Tree Zigzag Level Order Traversal.
    // https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<Vec<i32>>, level: usize) {
            if node.is_none() {
                return;
            }
            if level == levels.len() {
                levels.push(Vec::new());
            }
            levels[level].push(RefCell::borrow(node.as_ref().unwrap()).val);
            dfs(
                RefCell::borrow(node.as_ref().unwrap()).left.clone(),
                levels,
                level + 1,
            );
            dfs(
                RefCell::borrow(node.as_ref().unwrap()).right.clone(),
                levels,
                level + 1,
            );
        }
        let mut levels: Vec<Vec<i32>> = Vec::new();
        dfs(root, &mut levels, 0);
        for i in (1..levels.len()).step_by(2) {
            levels[i].reverse();
        }
        levels
    }

    // 260. Single Number III.
    // https://leetcode.com/problems/single-number-iii/
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1
        }
        let mut res: Vec<i32> = Vec::with_capacity(2);
        for (n, c) in map {
            if c == 1 {
                res.push(n);
            }
        }
        res
    }

    pub fn single_number_v2(nums: Vec<i32>) -> Vec<i32> {
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len() / 2);
        for n in nums {
            if set.contains(&n) {
                set.remove(&n);
            } else {
                set.insert(n);
            }
        }
        set.into_iter().collect()
    }

    // 797. All Paths From Source to Target.
    // https://leetcode.com/problems/all-paths-from-source-to-target/
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(graph: &Vec<Vec<i32>>, res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
            let target = graph.len() as i32 - 1;
            let last = *path.last().unwrap();
            if last == target {
                res.push(path.clone());
                return;
            }
            for v in &graph[last as usize] {
                path.push(*v);
                dfs(graph, res, path);
                path.pop();
            }
        }
        let mut res = Vec::<Vec<i32>>::new();
        let mut path = Vec::<i32>::with_capacity(graph.len());
        path.push(0);
        dfs(&graph, &mut res, &mut path);
        res
    }

    // 258. Add Digits.
    // https://leetcode.com/problems/add-digits/
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            0
        } else {
            1 + (num - 1) % 9
        }
    }

    // 106. Construct Binary Tree from Inorder and Postorder Traversal.
    // https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_bt(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if inorder.is_empty() || postorder.is_empty() {
                return None;
            }
            let val = *postorder.last().unwrap();
            let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            if postorder.len() == 1 {
                return node;
            }
            let i = inorder.iter().position(|x| *x == val).unwrap();
            RefCell::borrow_mut(node.as_ref().unwrap()).left =
                build_bt(&inorder[..i], &postorder[0..i]);
            RefCell::borrow_mut(node.as_ref().unwrap()).right =
                build_bt(&inorder[i + 1..], &postorder[i..postorder.len() - 1]);
            node
        }

        build_bt(&inorder, &postorder)
    }

    // 621. Task Scheduler.
    // https://leetcode.com/problems/task-scheduler/
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut task_counter = vec![0i32; 26];
        for t in &tasks {
            let i = *t as usize;
            task_counter[i - 65] += 1;
        }
        task_counter.sort_unstable();
        let f_max_1 = task_counter.pop().unwrap() - 1;
        let mut idle_time = f_max_1 * n;
        while !task_counter.is_empty() && idle_time > 0 {
            idle_time -= f_max_1.min(task_counter.pop().unwrap());
        }
        tasks.len() as i32 + idle_time
    }

    // 140. Word Break II.
    // https://leetcode.com/problems/word-break-ii/
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        if s.is_empty() || word_dict.is_empty() {
            return vec![];
        }

        let word_set: HashSet<&str> = HashSet::from_iter(word_dict.iter().map(|x| x.as_str()));
        let mut memo: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();

        fn solve<'a>(
            word_set: &'a HashSet<&'a str>,
            memo: &mut HashMap<&'a str, Vec<Vec<&'a str>>>,
            s: &'a str,
        ) -> Vec<Vec<&'a str>> {
            if s.is_empty() {
                // It is crucial to return vec of an empty vec, not just an empty vec.
                // The caller will extend it with a word.
                return vec![Vec::with_capacity(1)];
            }

            if let Entry::Occupied(o) = memo.entry(s) {
                return o.get().clone();
            }
            let mut sentences: Vec<Vec<&str>> = Vec::new();
            for end_index in 1..=s.len() {
                let word = &s[..end_index];
                if word_set.contains(word) {
                    for mut subsentences in solve(word_set, memo, &s[end_index..]) {
                        subsentences.push(word);
                        sentences.push(subsentences);
                    }
                }
            }
            memo.insert(s, sentences.clone());
            sentences
        }

        solve(&word_set, &mut memo, &s)
            .into_iter()
            .map(|list| {
                list.into_iter()
                    .rev()
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .to_string()
            })
            .collect()
    }

    // 70. Climbing Stairs.
    // https://leetcode.com/problems/climbing-stairs/
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 2 {
            return 1;
        }
        let mut f0 = 1;
        let mut f1 = 1;
        for _ in 2..=n {
            let t = f1;
            f1 += f0;
            f0 = t;
        }
        f1
    }
}
