mod pick_index;
mod randomized_set;
mod randomized_set_v2;

#[cfg(test)]
mod tests;

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::char;
use std::collections::BinaryHeap;

use crate::common::tree_node::TreeNode;

struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(some) = root {
            let left = RefCell::borrow_mut(&some).left.clone();
            let right = RefCell::borrow_mut(&some).right.clone();
            if left.is_none() && right.is_some() {
                RefCell::borrow_mut(&some).left = right;
                RefCell::borrow_mut(&some).right = None;
                Solution::invert_tree(RefCell::borrow_mut(&some).left.clone());
            } else if left.is_some() && right.is_none() {
                RefCell::borrow_mut(&some).right = left;
                RefCell::borrow_mut(&some).left = None;
                Solution::invert_tree(RefCell::borrow_mut(&some).right.clone());
            } else if left.is_some() && right.is_some() {
                let node = RefCell::borrow(&some);
                node.left.as_ref().unwrap().swap(node.right.as_ref().unwrap());
                Solution::invert_tree(node.left.clone());
                Solution::invert_tree(node.right.clone());
            }
            Some(some)
        } else {
            None
        }
    }

    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_unstable_by_key(|a| a[0] - a[1]);
        let n = costs.len() / 2;
        costs.iter().take(n).map(|x| x[0]).sum::<i32>() + costs.iter().skip(n).map(|x| x[1]).sum::<i32>()
    }

    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() { return; }
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|x, y| match x[0].cmp(&y[0]) {
            Ordering::Equal => x[1].cmp(&y[1]),
            o => o,
        });
        let mut res: Vec<Vec<i32>> = vec![vec![]; people.len()];
        for p in people.iter() {
            let mut cnt = p[1];
            let mut i = 0usize;
            loop {
                if res[i].is_empty() || res[i][0] >= p[0] {
                    if cnt == 0 {
                        break;
                    }
                    cnt -= 1;
                }
                i += 1
            }
            res[i] = p.clone();
        }
        res
    }

    pub fn reconstruct_queue_v2(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|x, y| match y[0].cmp(&x[0]) {
            Ordering::Equal => x[1].cmp(&y[1]),
            o => o,
        });
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(people.len());
        for p in people.iter() {
            res.insert(p[1] as usize, p.clone());
        }
        res
    }

    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        if amount == 0 { return 1; }
        if coins.is_empty() { return 0; }
        let mut table = vec![vec![0; amount as usize]; coins.len()];
        for row in 0..coins.len() {
            for col in 0..amount as usize {
                let amt = col as i32 + 1;
                let cn = coins[row];
                table[row][col] = match amt - cn {
                    0 => 1,
                    d if d > 0 => table[row][d as usize - 1],
                    _ => 0
                } + if row == 0 { 0 } else { table[row - 1][col] };
            }
        }
        table[coins.len() - 1][amount as usize - 1]
    }

    pub fn is_power_of_two(mut n: i32) -> bool {
        if n <= 0 { return false; }
        loop {
            if n == 1 {
                return true;
            }
            if n % 2 == 1 {
                return false;
            }
            n /= 2;
        }
    }

    pub fn is_power_of_two_v2(n: i32) -> bool {
        n > 0 && n == n & -n
    }

    pub fn is_power_of_two_v3(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }

    pub fn is_subsequence(s: String, t: String) -> bool {
        if !s.is_empty() && t.is_empty() { return false; }
        if s.is_empty() { return true; }
        let mut i = 0;
        let s_chars: Vec<char> = s.chars().collect();
        for c in t.chars() {
            if s_chars[i] == c {
                i += 1;
                if i == s.len() {
                    return true;
                }
            }
        }
        i == s.len()
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        (match nums.binary_search(&target) {
            Ok(i) => i,
            Err(i) => i,
        }) as i32
    }

    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() < 2 { return; }
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut i = 0;
        while i <= right {
            let n = nums[i];
            match n {
                0 => {
                    nums.swap(i, left);
                    left += 1;
                    i += 1;
                },
                2 => {
                    nums.swap(i, right);
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                },
                _ => i += 1,
            }
        }
    }

    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 { return nums; }
        let mut div_count: Vec<usize> = vec![1; nums.len()];
        let mut prev: Vec<isize> = vec![-1; nums.len()];
        nums.sort_unstable();
        let mut max_idx = 0;
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && div_count[i] < div_count[j] + 1 {
                    prev[i] = j as isize;
                    div_count[i] = div_count[j] + 1;
                }
            }
            if div_count[max_idx] < div_count[i] {
                max_idx = i;
            }
        }
        let mut res: Vec<i32> = Vec::new();
        let mut i  = max_idx as isize;
        while i >= 0 {
            res.push(nums[i as usize]);
            i = prev[i as usize];
        }
        res
    }

    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        //
        // Dijkstra's algorithm
        //

        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            node: usize,
            cost: usize,
            stops: usize,
        }

        // Manually implement Ord so we get a min-heap instead of a max-heap
        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other.cost.cmp(&self.cost)
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut adj_mx: Vec<Vec<_>> = vec![vec![0usize; n as usize]; n as usize];
        for f in flights.iter() {
            adj_mx[f[0] as usize][f[1] as usize] = f[2] as usize;
        }
        let mut heap = BinaryHeap::new();
        let mut visited = vec![-1i32; n as usize];
        heap.push(State {
            node: src as usize,
            cost: 0,
            stops: k as usize + 1,
        });
        while let Some(State { node, cost, stops}) = heap.pop() {
            if node == dst as usize {
                return cost as i32;
            }
            visited[node] = stops as i32;
            if stops > 0 {
                for i in 0..adj_mx[node].len() {
                    if adj_mx[node][i] <= 0 || visited[i] >= stops as i32 - 1 { continue; }
                    let edge_cost = adj_mx[node][i];
                    let next = State {
                        node: i,
                        cost: cost + edge_cost,
                        stops: stops - 1,
                    };
                    heap.push(next);
                }
            }
        }
        -1
    }

    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(some) = root {
            match RefCell::borrow(&some).val.cmp(&val)  {
                Ordering::Greater => Solution::search_bst(RefCell::borrow(&some).left.clone(), val),
                Ordering::Less => Solution::search_bst(RefCell::borrow(&some).right.clone(), val),
                Ordering::Equal => Some(some.clone()),
            }
        } else {
            None
        }
    }

    pub fn valid_ip_address(ip: String) -> String {
        let neither = "Neither".to_string();
        let ip_v4 = "IPv4".to_string();
        let ip_v6 = "IPv6".to_string();

        // Parsing IPv4 address.
        if ip.contains('.') {
            let parts = ip.split('.').collect::<Vec<&str>>();
            if parts.len() != 4 {
                return neither;
            }
            for p in parts.iter() {
                if p.len() > 1 && p.starts_with('0') {
                    return neither;
                }
                if let Ok(n) = u32::from_str_radix(p, 10) {
                    if n > 255 {
                        return neither;
                    }
                } else {
                    return neither;
                }
            }
            return ip_v4;
        }

        // Parsing IPv6 address.
        if ip.contains(':') {
            let parts = ip.split(':').collect::<Vec<&str>>();
            if parts.len() != 8 {
                return neither;
            }
            for p in parts.iter() {
                if p.is_empty() || p.len() > 4 {
                    return neither;
                }
                if let Err(_) = u32::from_str_radix(p, 16) {
                    return neither;
                }
            }
            return ip_v6;
        }
        neither
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() < 3 || board[0].len() < 3 { return; };

        let mut reachable_from_edges: Vec<Vec<_>> = vec![vec![false; board[0].len()]; board.len()];
        fn visit(board: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, row: isize, col: isize) {
            let r = row as usize;
            let c = col as usize;
            if row < 0 || r >= board.len() || col < 0 || c >= board[r].len()
                || visited[r][c] || board[r][c] != 'O' { return; }
            visited[r][c] = true;
            visit(board, visited, row + 1, col);
            visit(board, visited, row - 1, col);
            visit(board, visited, row, col + 1);
            visit(board, visited, row, col - 1);
        }

        // Visiting cells on the left & right edges.
        for row in 0..board.len() {
            visit(board, &mut reachable_from_edges, row as isize, 0);
            visit(board, &mut reachable_from_edges, row as isize, board[row].len() as isize - 1);
        }

        // Visiting cells on the top & bottom edges.
        for col in 1..board[0].len() - 1 {
            visit(board, &mut reachable_from_edges, 0, col as isize);
            visit(board, &mut reachable_from_edges, board.len() as isize - 1, col as isize);
        }

        // Flipping O-chars which are not reachable from the edges.
        for row in 1..board.len() - 1 {
            for col in 1..board[row].len() - 1 {
                if board[row][col] == 'O' && !reachable_from_edges[row][col] {
                    board[row][col] = 'X';
                }
            }
        }
   }

    // ~Log N
    pub fn h_index_ii(citations: Vec<i32>) -> i32 {
        if citations.is_empty() { return 0; }
        let l = citations.len() as i32;
        let mut left= 0;
        let mut right = l - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let h = l - mid;
            match citations[mid as usize].cmp(&h) {
                Ordering::Equal => return h,
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
            }
        }
        l - left
    }

    // ~N
    pub fn h_index_ii_v2(citations: Vec<i32>) -> i32 {
        for i in 0..citations.len() {
            let h = citations.len() - i;
            if citations[i] as usize >= h {
                return h as i32;
            }
        }
        0
    }

    pub fn get_permutation(n: i32, k: i32) -> String {
        if n == 1 { return "1".to_string(); }
        let mut res: Vec<usize> = Vec::with_capacity(n as usize);
        let block_sizes: [usize; 8] = [1, 2, 6, 24, 120, 720, 5040, 40320];
        let mut digits: Vec<usize> = (1..=n as usize).collect();
        let mut curr_k = k as usize;
        for i in (0..n as usize - 1).rev() {
            let block_size = block_sizes[i];
            let digit_index = curr_k / block_size - if curr_k > 0 && curr_k % block_size == 0 { 1 } else { 0 };
            curr_k -= block_size * digit_index;
            res.push(digits.remove(digit_index));
        }
        res.push(digits.pop().unwrap());
        res.iter().map(|x| char::from_digit(*x as u32, 10).unwrap()).collect()
    }
}