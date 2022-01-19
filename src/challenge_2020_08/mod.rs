use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use crate::common::list_node::ListNode;
use crate::common::tree_node::TreeNode;

mod hash_set;
mod logger_v1;
mod logger_v2;
mod trie_node;
mod word_dictionary;

#[cfg(test)]
mod tests;

struct Solution;

impl Solution {
    // 520. Detect Capital.
    // https://leetcode.com/problems/detect-capital/
    pub fn detect_capital_use(word: String) -> bool {
        let mut is_upper_allowed = true;
        let mut is_lower_allowed = true;
        for (i, c) in word.chars().enumerate() {
            match (c.is_ascii_uppercase(), is_upper_allowed, is_lower_allowed) {
                (false, _, true) => {
                    is_upper_allowed = false;
                }
                (true, false, _) | (false, _, false) => {
                    return false;
                }
                (true, _, _) if i == 1 => {
                    is_lower_allowed = false;
                }
                _ => (),
            }
        }
        true
    }

    // 125. Valid Palindrome.
    // https://leetcode.com/problems/valid-palindrome/
    // Iterative method.
    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn is_palindrome(s: String) -> bool {
        if s.len() < 2 {
            return true;
        }
        let mut left = 0;
        let mut right = s.len() - 1;
        let chars: Vec<_> = s.chars().collect();
        loop {
            while left < right && !chars[left].is_alphanumeric() {
                left += 1;
            }
            while left < right && !chars[right].is_alphanumeric() {
                right -= 1;
            }
            if !chars[left].eq_ignore_ascii_case(&chars[right]) {
                return false;
            }
            if left >= right {
                return true;
            }
            left += 1;
            right -= 1;
        }
    }

    // 342. Power of Four.
    // https://leetcode.com/problems/power-of-four/
    pub fn is_power_of_four(mut num: i32) -> bool {
        if num == 1 {
            return true;
        }
        if num < 4 || num & (num - 1) != 0 {
            return false;
        }
        let mut count = 0;
        while num > 1 {
            num >>= 1;
            count += 1;
        }
        count % 2 == 0
    }

    pub fn is_power_of_four_v2(num: i32) -> bool {
        let is_power_of_two = num > 0 && num & (-num) == num;
        is_power_of_two && (num & 0b1010101010101010101010101010101 != 0)
    }

    pub fn is_power_of_four_v3(num: i32) -> bool {
        let is_power_of_two = num > 0 && num & (-num) == num;
        is_power_of_two && num % 3 == 1
    }

    // 442. Find All Duplicates in an Array.
    // https://leetcode.com/problems/find-all-duplicates-in-an-array/
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut set: HashSet<i32> = HashSet::with_capacity(nums.len() / 2);
        for n in nums {
            if set.contains(&n) {
                set.remove(&n);
                res.push(n);
            } else {
                set.insert(n);
            }
        }
        res
    }

    pub fn find_duplicates_v2(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            let n = nums[i].abs();
            let j = n as usize - 1;
            if nums[j] > 0 {
                nums[j] = -nums[j];
            } else {
                res.push(n);
            }
        }
        res
    }

    // 987. Vertical Order Traversal of a Binary Tree.
    // https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            x: i32,
            y: i32,
            map: &mut HashMap<i32, Vec<(i32, i32)>>,
        ) {
            if let Some(some) = node {
                map.entry(x)
                    .or_insert(Vec::new())
                    .push((y, RefCell::borrow(&some).val));
                dfs(&RefCell::borrow(&some).left, x - 1, y - 1, map);
                dfs(&RefCell::borrow(&some).right, x + 1, y - 1, map);
            }
        }

        let mut map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        dfs(&root, 0, 0, &mut map);
        let mut vec: Vec<_> = map.into_iter().map(|(x, a)| (x, a)).collect();
        vec.sort_unstable_by_key(|a| a.0);
        vec.into_iter()
            .map(|(_, mut yv)| {
                yv.sort_unstable_by(|(y1, v1), (y2, v2)| match y2.cmp(y1) {
                    Ordering::Equal => v1.cmp(v2),
                    a => a,
                });
                yv.into_iter().map(|(_, v)| v).collect()
            })
            .collect()
    }

    pub fn vertical_traversal_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            x: i32,
            y: i32,
            list: &mut Vec<(i32, i32, i32)>,
        ) {
            if let Some(some) = node {
                list.push((x, y, RefCell::borrow(&some).val));
                dfs(&RefCell::borrow(&some).left, x - 1, y - 1, list);
                dfs(&RefCell::borrow(&some).right, x + 1, y - 1, list);
            }
        }

        let mut list = Vec::new();
        dfs(&root, 0, 0, &mut list);
        list.sort_unstable_by(|(x1, y1, v1), (x2, y2, v2)| match x1.cmp(x2) {
            Ordering::Equal => match y2.cmp(y1) {
                Ordering::Equal => v1.cmp(v2),
                a => a,
            },
            b => b,
        });
        let mut slice_start = 0;
        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in 1..=list.len() {
            if i == list.len() || list[i].0 != list[i - 1].0 {
                res.push(list[slice_start..i].iter().map(|(_, _, v)| *v).collect());
                slice_start = i;
            }
        }
        res
    }

    // 270. Closest Binary Search Tree Value.
    // https://leetcode.com/problems/closest-binary-search-tree-value/
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        fn get_closest_val(val_1: i32, val_2: i32, target: f64) -> i32 {
            if (val_1 as f64 - target).abs() < (val_2 as f64 - target).abs() {
                val_1
            } else {
                val_2
            }
        }

        fn dfs(node: &Rc<RefCell<TreeNode>>, target: f64) -> i32 {
            let n = RefCell::borrow(node);
            if ((n.val as f64) - target).abs() > 0.5 {
                if (n.val as f64) > target && n.left.is_some() {
                    get_closest_val(dfs(n.left.as_ref().unwrap(), target), n.val, target)
                } else if (n.val as f64) < target && n.right.is_some() {
                    get_closest_val(dfs(n.right.as_ref().unwrap(), target), n.val, target)
                } else {
                    n.val
                }
            } else {
                n.val
            }
        }

        dfs(root.as_ref().unwrap(), target)
    }

    pub fn closest_value_v2(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        let mut closest = RefCell::borrow(root.as_ref().unwrap()).val;
        let mut diff = (closest as f64 - target).abs();
        let mut node = root;
        while let Some(some) = node {
            let n = RefCell::borrow(&some);
            if (n.val as f64 - target).abs() < diff {
                closest = n.val;
                diff = (closest as f64 - target).abs();
            }
            if target < n.val as f64 {
                node = n.left.clone();
            } else if target > n.val as f64 {
                node = n.right.clone();
            } else {
                break;
            }
        }
        closest
    }

    // 437. Path Sum III.
    // https://leetcode.com/problems/path-sum-iii/
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            k: i32,
            mut sum: i32,
            sums: &mut HashMap<i32, i32>,
        ) -> i32 {
            if let Some(some) = node {
                let n = RefCell::borrow(&some);
                sum += n.val;
                let key = sum - k;
                let mut cnt = *sums.get(&key).unwrap_or(&0);
                *sums.entry(sum).or_insert(0) += 1;
                cnt += dfs(n.left.clone(), k, sum, sums) + dfs(n.right.clone(), k, sum, sums);
                *sums.get_mut(&sum).unwrap() -= 1;
                cnt
            } else {
                0
            }
        }

        let mut sums: HashMap<i32, i32> = HashMap::new();
        sums.insert(0, 1);
        dfs(root, k, 0, &mut sums)
    }

    // 994. Rotting Oranges.
    // https://leetcode.com/problems/rotting-oranges/
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return -1;
        }
        let mut fresh_cnt = 0usize;
        let mut rotten_oranges: VecDeque<(isize, isize)> = VecDeque::new();
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                match grid[r][c] {
                    1 => fresh_cnt += 1,
                    2 => rotten_oranges.push_back((r as isize, c as isize)),
                    _ => (),
                }
            }
        }
        let mut step = 0;
        loop {
            for _ in 0..rotten_oranges.len() {
                let (r, c) = rotten_oranges.pop_front().unwrap();
                for (y, x) in vec![(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                    let are_xy_valid = y >= 0
                        && y < grid.len() as isize
                        && x >= 0
                        && x < grid[y as usize].len() as isize;
                    if are_xy_valid && grid[y as usize][x as usize] == 1 {
                        grid[y as usize][x as usize] = 2;
                        fresh_cnt -= 1;
                        rotten_oranges.push_back((y, x));
                    }
                }
            }
            if rotten_oranges.is_empty() {
                break;
            }
            step += 1;
        }
        if fresh_cnt == 0 {
            step
        } else {
            -1
        }
    }

    // 171. Excel Sheet Column Number
    // https://leetcode.com/problems/excel-sheet-column-number/
    pub fn title_to_number(s: String) -> i32 {
        s.chars().fold(0, |acc, c| acc * 26 + (c as i32 - 64))
    }

    // 119. Pascal's Triangle II.
    // https://leetcode.com/problems/pascals-triangle-ii/
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row: Vec<i32> = Vec::with_capacity(row_index as usize + 1);
        row.push(1);
        for i in 0..row_index {
            row.push((row[i as usize] as u64 * (row_index - i) as u64 / (i + 1) as u64) as i32);
        }
        row
    }

    // 409. Longest Palindrome.
    // https://leetcode.com/problems/longest-palindrome/
    pub fn longest_palindrome(s: String) -> i32 {
        let mut char_counter = vec![0; 52];
        for c in s.chars() {
            let mut i = c as usize;
            i = if i > 90 { i - 71 } else { i - 65 };
            char_counter[i] += 1;
        }
        let mut counter = 0;
        for cnt in char_counter {
            counter += cnt / 2 * 2;
            if counter % 2 == 0 && cnt % 2 == 1 {
                counter += 1;
            }
        }
        counter
    }

    // 123. Best Time to Buy and Sell Stock III.
    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
    // Iterative DP method.
    // Time complexity: O(N).
    // Space complexity: O(N).
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let l = prices.len();
        let mut left_min_price = prices[0];
        let mut right_min_price = prices[l - 1];
        let mut left_profits = vec![0; l];
        // Pad the right DP array with an additional zero for convenience.
        let mut right_profits = vec![0; l + 1];
        for i in 1..l {
            left_profits[i] = left_profits[i - 1].max(prices[i] - left_min_price);
            left_min_price = left_min_price.min(prices[i]);
            let j = l - i - 1;
            right_profits[j] = right_profits[j + 1].max(right_min_price - prices[j]);
            right_min_price = right_min_price.min(prices[j]);
        }
        let mut max_profit = 0;
        for i in 0..l {
            max_profit = max_profit.max(left_profits[i] + right_profits[i + 1]);
        }
        max_profit
    }

    // Time complexity: O(N).
    // Space complexity: O(1).
    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut t1_cost = i32::MAX;
        let mut t2_cost = i32::MAX;
        let mut t1_profit = 0;
        let mut t2_profit = 0;
        for i in 0..prices.len() {
            t1_cost = t1_cost.min(prices[i]);
            t1_profit = t1_profit.max(prices[i] - t1_cost);
            t2_cost = t2_cost.min(prices[i] - t1_profit);
            t2_profit = t2_profit.max(prices[i] - t2_cost);
        }
        t2_profit
    }

    // 824. Goat Latin.
    // https://leetcode.com/problems/goat-latin/
    pub fn to_goat_latin(s: String) -> String {
        let mut transformed_words: Vec<String> = Vec::new();
        for (i, w) in s.split_ascii_whitespace().enumerate() {
            let mut word_chars: VecDeque<_> = w.chars().collect();
            match word_chars[0] {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    word_chars.extend(&['m', 'a'])
                }
                _ => {
                    let first = word_chars.pop_front().unwrap();
                    word_chars.extend(&[first, 'm', 'a'])
                }
            }
            word_chars.extend(vec!['a'; i + 1]);
            transformed_words.push(word_chars.iter().collect());
        }
        transformed_words.join(" ")
    }

    // 143. Reorder List.
    // https://leetcode.com/problems/reorder-list/
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        if head.as_ref().unwrap().next.is_none() {
            return;
        }

        fn get_length(head: &Option<Box<ListNode>>) -> usize {
            let mut res = 0;
            let mut current_node = head;
            while current_node.is_some() {
                current_node = &current_node.as_ref().unwrap().next;
                res += 1;
            }
            res
        }

        fn split(
            mut head: Option<Box<ListNode>>,
            len: usize,
        ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
            let mut curr_node = &mut head;
            for _ in 0..len {
                let curr_node_inner = curr_node.as_mut().unwrap();
                curr_node = &mut curr_node_inner.next;
            }
            let tail = curr_node.take();
            (head, tail)
        }

        fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev_node = None;
            let mut head = head;
            let mut curr_node = head.take();

            while let Some(mut current_node_inner) = curr_node.take() {
                let next_node = current_node_inner.next.take();
                current_node_inner.next = prev_node.take();
                prev_node = Some(current_node_inner);
                curr_node = next_node;
            }

            prev_node.take()
        }

        fn merge(
            head_1: Option<Box<ListNode>>,
            head_2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let mut head_3: Box<ListNode> = Box::new(ListNode::new(0)); // Sentinel node.
            let mut prev_node: &mut Box<ListNode> = &mut head_3;
            let mut node_1 = head_1;
            let mut node_2 = head_2;

            while let Some(mut node_1_inner) = node_1 {
                node_1 = node_1_inner.next.take();
                if let Some(mut node_2_inner) = node_2 {
                    node_2 = node_2_inner.next.take();
                    node_1_inner.next = Some(node_2_inner);
                    prev_node.next = Some(node_1_inner);
                    prev_node = prev_node.next.as_mut().unwrap().next.as_mut().unwrap();
                } else {
                    prev_node.next = Some(node_1_inner);
                    prev_node = prev_node.next.as_mut().unwrap();
                    node_2 = None;
                }
            }
            head_3.next.take()
        }

        let node = head.take();
        let len = get_length(&node);
        let (head_1, mut head_2) = split(node, len / 2 + len % 2);
        head_2 = reverse(head_2);
        head.replace(merge(head_1, head_2).unwrap());
    }

    // 905. Sort Array By Parity.
    // https://leetcode.com/problems/sort-array-by-parity/
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        a.sort_unstable_by(|x, y| match (x % 2 == 0, y % 2 == 0) {
            (true, true) | (false, false) => x.cmp(y),
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
        });
        a
    }

    pub fn sort_array_by_parity_v2(mut a: Vec<i32>) -> Vec<i32> {
        a.sort_by_key(|k| k % 2);
        a
    }

    // 490. The Maze.
    // https://leetcode.com/problems/the-maze/
    // My first solution. It is more complex dfs, but
    // 1) it doesn't use extra space,
    // 2) it doesn't rolls backward,
    // 3) it stops rolling further if the stop point lays on a visited cell.
    pub fn has_path(mut maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        #[derive(PartialEq, Eq, Clone, Debug)]
        enum Direction {
            X,
            Y,
        }

        fn find_directions(
            maze: &Vec<Vec<i32>>,
            r: usize,
            c: usize,
            prev_direction: Option<Direction>,
        ) -> Vec<(Direction, i32)> {
            let mut res: Vec<(Direction, i32)> = Vec::with_capacity(4);
            if prev_direction.is_none() || *prev_direction.as_ref().unwrap() != Direction::Y {
                if r > 0 && maze[r - 1][c] != 1 {
                    res.push((Direction::Y, -1));
                }
                if r < maze.len() - 1 && maze[r + 1][c] != 1 {
                    res.push((Direction::Y, 1));
                }
            }

            if prev_direction.is_none() || *prev_direction.as_ref().unwrap() != Direction::X {
                if c > 0 && maze[r][c - 1] != 1 {
                    res.push((Direction::X, -1));
                }
                if c < maze[r].len() - 1 && maze[r][c + 1] != 1 {
                    res.push((Direction::X, 1));
                }
            }
            res
        }

        fn roll_along_x(
            maze: &mut Vec<Vec<i32>>,
            r: usize,
            mut c: usize,
            step: i32,
        ) -> (usize, usize, i32) {
            let mut last_value;
            loop {
                last_value = maze[r][c];
                if maze[r][c] != 3 {
                    maze[r][c] = 2;
                }
                let new_c = c as i32 + step;
                if (new_c < 0 || new_c > maze[r].len() as i32 - 1) || maze[r][new_c as usize] == 1 {
                    maze[r][c] = 3;
                    break;
                }
                c = new_c as usize;
            }
            (r, c, last_value)
        }

        fn roll_along_y(
            maze: &mut Vec<Vec<i32>>,
            mut r: usize,
            c: usize,
            step: i32,
        ) -> (usize, usize, i32) {
            let mut last_value;
            loop {
                last_value = maze[r][c];
                if maze[r][c] != 3 {
                    maze[r][c] = 2;
                }
                let new_r = r as i32 + step;
                if (new_r < 0 || new_r > maze.len() as i32 - 1) || maze[new_r as usize][c] == 1 {
                    maze[r][c] = 3;
                    break;
                }
                r = new_r as usize;
            }
            (r, c, last_value)
        }

        fn solve(
            maze: &mut Vec<Vec<i32>>,
            r: usize,
            c: usize,
            dest_r: usize,
            dest_c: usize,
            prev_direction: Option<Direction>,
        ) -> bool {
            let dirs = find_directions(&maze, r, c, prev_direction);
            for (dir, step) in dirs {
                let (stop_r, stop_c, last_value) = match (&dir, step) {
                    (Direction::X, s) => roll_along_x(maze, r, c, s),
                    (Direction::Y, s) => roll_along_y(maze, r, c, s),
                };

                if stop_r == dest_r && stop_c == dest_c {
                    return true;
                }
                if last_value == 0 && solve(maze, stop_r, stop_c, dest_r, dest_c, Some(dir)) {
                    return true;
                }
            }
            false
        }

        solve(
            &mut maze,
            start[0] as usize,
            start[1] as usize,
            destination[0] as usize,
            destination[1] as usize,
            None,
        )
    }

    // Simple dfs-solution.
    pub fn has_path_v2(mut maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; maze[0].len()]; maze.len()];
        fn dfs(
            maze: &Vec<Vec<i32>>,
            start_r: usize,
            start_c: usize,
            dest_r: usize,
            dest_c: usize,
            visited: &mut Vec<Vec<bool>>,
        ) -> bool {
            if visited[start_r][start_c] {
                return false;
            }

            if start_r == dest_r && start_c == dest_c {
                return true;
            }
            visited[start_r][start_c] = true;

            // Right.
            let mut r = start_c as i32 + 1;
            while r < maze[start_r].len() as i32 && maze[start_r][r as usize] == 0 {
                r += 1;
            }
            if dfs(maze, start_r, (r - 1) as usize, dest_r, dest_c, visited) {
                return true;
            }

            // Left.
            let mut l = start_c as i32 - 1;
            while l >= 0 && maze[start_r][l as usize] == 0 {
                l -= 1;
            }
            if dfs(maze, start_r, (l + 1) as usize, dest_r, dest_c, visited) {
                return true;
            }

            // Up.
            let mut u = start_r as i32 - 1;
            while u >= 0 && maze[u as usize][start_c] == 0 {
                u -= 1;
            }
            if dfs(maze, (u + 1) as usize, start_c, dest_r, dest_c, visited) {
                return true;
            }

            // Down.
            let mut d = start_r as i32 + 1;
            while d < maze.len() as i32 && maze[d as usize][start_c] == 0 {
                d += 1;
            }
            if dfs(maze, (d - 1) as usize, start_c, dest_r, dest_c, visited) {
                return true;
            }

            false
        }

        dfs(
            &mut maze,
            start[0] as usize,
            start[1] as usize,
            destination[0] as usize,
            destination[1] as usize,
            &mut visited,
        )
    }

    // 404. Sum of Left Leaves.
    // https://leetcode.com/problems/sum-of-left-leaves/
    // Recursive solution.
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            let node_inner = node.as_ref().unwrap();
            let node_borrowed = RefCell::borrow(&node_inner);
            if node_borrowed.left.is_none() && node_borrowed.right.is_none() {
                if is_left {
                    node_borrowed.val
                } else {
                    0
                }
            } else {
                let mut sum = 0;
                if node_borrowed.left.is_some() {
                    sum += dfs(&node_borrowed.left, true);
                }
                if node_borrowed.right.is_some() {
                    sum += dfs(&node_borrowed.right, false);
                }
                sum
            }
        }
        dfs(&root, false)
    }

    // Iterative solution.
    pub fn sum_of_left_leaves_v2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![];
        let mut sum = 0;

        stack.push((root, false)); // is_left: false
        while let Some((node, is_left)) = stack.pop() {
            if let Some(node_inner) = node {
                let node_borrowed = node_inner.borrow();
                if is_left && node_borrowed.left.is_none() && node_borrowed.right.is_none() {
                    sum += node_borrowed.val;
                }
                stack.push((node_borrowed.left.clone(), true));
                stack.push((node_borrowed.right.clone(), false));
            }
        }

        sum
    }

    // 412. Fizz Buzz.
    // https://leetcode.com/problems/fizz-buzz/
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::with_capacity(n as usize);
        let buzz = "Buzz".to_string();
        let fizz = "Fizz".to_string();
        let fizz_buzz = "FizzBuzz".to_string();
        for i in 1..=n {
            let is_mul_of_5 = i % 5 == 0;
            let is_mul_of_3 = i % 3 == 0;
            match (is_mul_of_5, is_mul_of_3) {
                (true, true) => res.push(fizz_buzz.clone()),
                (true, false) => res.push(buzz.clone()),
                (false, true) => res.push(fizz.clone()),
                _ => res.push(i.to_string()),
            }
        }
        res
    }

    // 436. Find Right Interval.
    // https://leetcode.com/problems/find-right-interval/
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        if intervals.len() == 1 {
            return vec![-1];
        }

        let mut indexed_intervals: Vec<(usize, Vec<i32>)> =
            intervals.into_iter().enumerate().collect();
        indexed_intervals.sort_unstable_by_key(|x| x.1[0]);

        let mut res: Vec<i32> = vec![-1; indexed_intervals.len()];
        for i in 0..indexed_intervals.len() - 1 {
            let mut j = i + 1;
            while j < indexed_intervals.len() {
                if indexed_intervals[i].1[1] <= indexed_intervals[j].1[0] {
                    res[indexed_intervals[i].0] = indexed_intervals[j].0 as i32;
                    break;
                }
                j += 1;
            }
        }
        res
    }

    // 450. Delete Node in a BST.
    // https://leetcode.com/problems/delete-node-in-a-bst/
    pub fn delete_node(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn find_min(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut curr = node.as_ref().unwrap().clone();
            while curr.borrow().left.is_some() {
                let tmp = curr.borrow().left.as_ref().unwrap().clone();
                curr = tmp;
            }
            let res = curr.borrow().val;
            res
        }

        fn remove(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
            if node.is_none() {
                return;
            }

            let node_val = node.as_ref().unwrap().borrow().val;
            match val.cmp(&node_val) {
                Ordering::Less => remove(&mut node.as_mut().unwrap().borrow_mut().left, val),
                Ordering::Greater => remove(&mut node.as_mut().unwrap().borrow_mut().right, val),
                _ => {
                    let has_left = node.as_ref().unwrap().borrow().left.is_some();
                    let has_right = node.as_ref().unwrap().borrow().right.is_some();

                    if !has_left && !has_right {
                        node.take();
                    } else if !has_right {
                        let left_node = node.as_mut().unwrap().borrow_mut().left.take().unwrap();
                        node.replace(left_node);
                    } else if !has_left {
                        let right_node = node.as_mut().unwrap().borrow_mut().right.take().unwrap();
                        node.replace(right_node);
                    } else {
                        let successor_val = find_min(&node.as_ref().unwrap().borrow().right);
                        node.as_mut().unwrap().borrow_mut().val = successor_val;
                        remove(
                            &mut node.as_mut().unwrap().borrow_mut().right,
                            successor_val,
                        );
                    }
                }
            }
        }

        remove(&mut root, val);
        root
    }
}
