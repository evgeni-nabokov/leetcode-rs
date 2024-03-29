mod pick_index;
mod randomized_set;
mod randomized_set_v2;

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::char;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter::FromIterator;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;

struct Solution;

impl Solution {
    // 226. Invert Binary Tree.
    // https://leetcode.com/problems/invert-binary-tree/
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
                node.left
                    .as_ref()
                    .unwrap()
                    .swap(node.right.as_ref().unwrap());
                Solution::invert_tree(node.left.clone());
                Solution::invert_tree(node.right.clone());
            }
            Some(some)
        } else {
            None
        }
    }

    // 1029. Two City Scheduling.
    // https://leetcode.com/problems/two-city-scheduling/
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_unstable_by_key(|a| a[0] - a[1]);
        let n = costs.len() / 2;
        costs.iter().take(n).map(|x| x[0]).sum::<i32>()
            + costs.iter().skip(n).map(|x| x[1]).sum::<i32>()
    }

    // 344. Reverse String.
    // https://leetcode.com/problems/reverse-string/
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() {
            return;
        }
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // 406. Queue Reconstruction by Height.
    // https://leetcode.com/problems/queue-reconstruction-by-height/
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|x, y| match x[0].cmp(&y[0]) {
            Ordering::Equal => x[1].cmp(&y[1]),
            o => o,
        });
        let mut res: Vec<Vec<i32>> = vec![vec![]; people.len()];
        for p in people {
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
            res[i] = p;
        }
        res
    }

    pub fn reconstruct_queue_v2(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|x, y| match y[0].cmp(&x[0]) {
            Ordering::Equal => x[1].cmp(&y[1]),
            o => o,
        });
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(people.len());
        for p in people {
            res.insert(p[1] as usize, p);
        }
        res
    }

    // 518. Coin Change 2.
    // https://leetcode.com/problems/coin-change-2/
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        if amount == 0 {
            return 1;
        }
        if coins.is_empty() {
            return 0;
        }
        let mut table = vec![vec![0; amount as usize]; coins.len()];
        for row in 0..coins.len() {
            for col in 0..amount as usize {
                let amt = col as i32 + 1;
                let cn = coins[row];
                table[row][col] = match amt - cn {
                    0 => 1,
                    d if d > 0 => table[row][d as usize - 1],
                    _ => 0,
                } + if row == 0 { 0 } else { table[row - 1][col] };
            }
        }
        table[coins.len() - 1][amount as usize - 1]
    }

    // 231. Power of Two.
    // https://leetcode.com/problems/power-of-two/
    pub fn is_power_of_two(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
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

    // 392. Is Subsequence.
    // https://leetcode.com/problems/is-subsequence/
    pub fn is_subsequence(s: String, t: String) -> bool {
        if !s.is_empty() && t.is_empty() {
            return false;
        }
        if s.is_empty() {
            return true;
        }
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

    // 35. Search Insert Position.
    // https://leetcode.com/problems/search-insert-position/
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        (match nums.binary_search(&target) {
            Ok(i) => i,
            Err(i) => i,
        }) as i32
    }

    // 75. Sort Colors.
    // https://leetcode.com/problems/sort-colors/.
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
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
                }
                2 => {
                    nums.swap(i, right);
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                }
                _ => i += 1,
            }
        }
    }

    // 368. Largest Divisible Subset.
    // https://leetcode.com/problems/largest-divisible-subset/
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return nums;
        }
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
        let mut i = max_idx as isize;
        while i >= 0 {
            res.push(nums[i as usize]);
            i = prev[i as usize];
        }
        res
    }

    // 787. Cheapest Flights Within K Stops.
    // https://leetcode.com/problems/cheapest-flights-within-k-stops/
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

        // Manually implement Ord so we get a min-heap instead of a max-heap.
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
        for f in flights {
            adj_mx[f[0] as usize][f[1] as usize] = f[2] as usize;
        }
        let mut heap = BinaryHeap::new();
        let mut visited = vec![-1i32; n as usize];
        heap.push(State {
            node: src as usize,
            cost: 0,
            stops: k as usize + 1,
        });
        while let Some(State { node, cost, stops }) = heap.pop() {
            if node == dst as usize {
                return cost as i32;
            }
            visited[node] = stops as i32;
            if stops > 0 {
                for i in 0..adj_mx[node].len() {
                    if adj_mx[node][i] <= 0 || visited[i] >= stops as i32 - 1 {
                        continue;
                    }
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

    // 700. Search in a Binary Search Tree.
    // https://leetcode.com/problems/search-in-a-binary-search-tree/
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(some) = root {
            match RefCell::borrow(&some).val.cmp(&val) {
                Ordering::Greater => Solution::search_bst(RefCell::borrow(&some).left.clone(), val),
                Ordering::Less => Solution::search_bst(RefCell::borrow(&some).right.clone(), val),
                Ordering::Equal => Some(some.clone()),
            }
        } else {
            None
        }
    }

    // 468. Validate IP Address.
    // https://leetcode.com/problems/validate-ip-address/
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
            for p in parts {
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
            for p in parts {
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

    // 130. Surrounded Regions.
    // https://leetcode.com/problems/surrounded-regions/
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() < 3 || board[0].len() < 3 {
            return;
        };

        let mut reachable_from_edges: Vec<Vec<_>> = vec![vec![false; board[0].len()]; board.len()];
        fn visit(board: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, row: isize, col: isize) {
            let r = row as usize;
            let c = col as usize;
            if row < 0
                || r >= board.len()
                || col < 0
                || c >= board[r].len()
                || visited[r][c]
                || board[r][c] != 'O'
            {
                return;
            }
            visited[r][c] = true;
            visit(board, visited, row + 1, col);
            visit(board, visited, row - 1, col);
            visit(board, visited, row, col + 1);
            visit(board, visited, row, col - 1);
        }

        // Visiting cells on the left & right edges.
        for row in 0..board.len() {
            visit(board, &mut reachable_from_edges, row as isize, 0);
            visit(
                board,
                &mut reachable_from_edges,
                row as isize,
                board[row].len() as isize - 1,
            );
        }

        // Visiting cells on the top & bottom edges.
        for col in 1..board[0].len() - 1 {
            visit(board, &mut reachable_from_edges, 0, col as isize);
            visit(
                board,
                &mut reachable_from_edges,
                board.len() as isize - 1,
                col as isize,
            );
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

    // 275. H-Index II.
    // https://leetcode.com/problems/h-index-ii/
    // ~Log N
    pub fn h_index_ii(citations: Vec<i32>) -> i32 {
        if citations.is_empty() {
            return 0;
        }
        let l = citations.len() as i32;
        let mut left = 0;
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

    // 1044. Longest Duplicate Substring
    // https://leetcode.com/problems/longest-duplicate-substring/
    // List of suffixes solution.
    // Explaining video: https://www.coursera.org/lecture/cs-algorithms-theory-machines/longest-repeated-substring-hkJBt
    // Not accepted - TLE.
    pub fn longest_dup_substring(s: String) -> String {
        if s.len() < 2 {
            return String::new();
        }

        fn get_lcp_len(s1: &[char], s2: &[char]) -> usize {
            let n = s1.len().min(s2.len());
            for i in 0..n {
                if s1[i] != s2[i] {
                    return i;
                }
            }
            n
        }

        let chars: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut suffixes: Vec<&[char]> = Vec::with_capacity(n);
        for i in 0..n {
            suffixes.push(&chars[i..]);
        }
        suffixes.sort_unstable();
        let mut lrs: &[char] = &vec![];
        for i in 1..n {
            let lcp_len = get_lcp_len(&suffixes[i - 1], &suffixes[i]);
            if lcp_len > lrs.len() {
                lrs = &suffixes[i][0..lcp_len];
            }
        }
        String::from_iter(lrs)
    }

    // Binary Search + Rabin-Karp solution.
    pub fn longest_dup_substring_v2(s: String) -> String {
        if s.len() < 2 {
            return String::new();
        }

        fn search_dup_substring(
            codes: &[u64],
            len: usize,
            base: u64,
            modulus: u64,
        ) -> Option<usize> {
            // Rolling hash.
            let mut hash = 0u64;
            // Const value to be used often: (base ^ len) % modulus.
            let mut base_pow_len = 1u64;
            // Compute the hash of first len codes and base_pow_len.
            for i in 0..len {
                hash = (hash * base + codes[i]) % modulus;
                base_pow_len = (base_pow_len * base) % modulus;
            }

            let hash_num = codes.len() - len;
            let mut seen: HashSet<u64> = HashSet::with_capacity(hash_num);
            seen.insert(hash);
            for start in 1..=hash_num {
                // Compute rolling hash in O(1) time.
                hash =
                    (hash * base - codes[start - 1] * base_pow_len % modulus + modulus) % modulus;
                hash = (hash + (codes[start + len - 1])) % modulus;
                if seen.contains(&hash) {
                    return Some(start);
                }
                seen.insert(hash);
            }
            None
        }

        let codes: Vec<_> = s.chars().map(|x| x as u64 - 97).collect();
        let mut left: i32 = 1;
        let mut right: i32 = s.len() as i32;
        let mut pointers: Option<(usize, usize)> = None;
        while left <= right {
            let mid = left + (right - left) / 2;
            if let Some(start) = search_dup_substring(&codes, mid as usize, 26, 4_294_967_296) {
                pointers = Some((start, start + mid as usize));
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        if let Some((start, end)) = pointers {
            s[start..end].to_string()
        } else {
            String::new()
        }
    }

    // O(N) time and O(1) space.
    pub fn h_index_ii_v2(citations: Vec<i32>) -> i32 {
        for i in 0..citations.len() {
            let h = citations.len() - i;
            if citations[i] as usize >= h {
                return h as i32;
            }
        }
        0
    }

    // 60. Permutation Sequence.
    // https://leetcode.com/problems/permutation-sequence/
    pub fn get_permutation(n: i32, k: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let mut res: Vec<usize> = Vec::with_capacity(n as usize);
        let block_sizes: [usize; 8] = [1, 2, 6, 24, 120, 720, 5040, 40320];
        let mut digits: Vec<usize> = (1..=n as usize).collect();
        let mut curr_k = k as usize;
        for i in (0..n as usize - 1).rev() {
            let block_size = block_sizes[i];
            let digit_index = curr_k / block_size
                - if curr_k > 0 && curr_k % block_size == 0 {
                    1
                } else {
                    0
                };
            curr_k -= block_size * digit_index;
            res.push(digits.remove(digit_index));
        }
        res.push(digits.pop().unwrap());
        res.iter()
            .map(|x| char::from_digit(*x as u32, 10).unwrap())
            .collect()
    }

    // 174. Dungeon Game.
    // https://leetcode.com/problems/dungeon-game/
    pub fn calculate_minimum_hp(mut dungeon: Vec<Vec<i32>>) -> i32 {
        if dungeon.is_empty() || dungeon[0].is_empty() {
            return 1;
        }
        let last_r = dungeon.len() - 1;
        let last_c = dungeon[0].len() - 1;
        for row in (0..=last_r).rev() {
            for col in (0..=last_c).rev() {
                dungeon[row][col] += if col == last_c && row == last_r {
                    0
                } else if col == last_c {
                    dungeon[row + 1][col]
                } else if row == last_r {
                    dungeon[row][col + 1]
                } else {
                    dungeon[row][col + 1].max(dungeon[row + 1][col])
                };
                if dungeon[row][col] > 0 {
                    dungeon[row][col] = 0;
                }
            }
        }
        dungeon[0][0].abs() + 1
    }

    // 137. Single Number II.
    // https://leetcode.com/problems/single-number-ii/
    pub fn single_number_ii(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut i = 0;
        loop {
            if i == nums.len() - 1 {
                return nums[i];
            }
            if nums[i] == nums[i + 1] && nums[i + 1] == nums[i + 2] {
                i += 3;
                continue;
            }
            if nums[i] == nums[i + 1] {
                return nums[i + 2];
            }
            if nums[i] == nums[i + 2] {
                return nums[i + 1];
            }
            if nums[i + 1] == nums[i + 2] {
                return nums[i];
            }
            break;
        }
        0
    }

    // 222. Count Complete Tree Nodes.
    // https://leetcode.com/problems/count-complete-tree-nodes/
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(some) = root {
            let mut left_height = 0;
            let mut left = RefCell::borrow(&some).left.clone();
            while left.is_some() {
                left_height += 1;
                let tmp = RefCell::borrow(left.as_ref().unwrap()).left.clone();
                left = tmp;
            }
            let mut right_height = 0;
            let mut right = RefCell::borrow(&some).right.clone();
            while right.is_some() {
                right_height += 1;
                let tmp = RefCell::borrow(right.as_ref().unwrap()).right.clone();
                right = tmp;
            }
            if left_height == right_height {
                2i32.pow(left_height + 1) - 1
            } else {
                1 + Solution::count_nodes(RefCell::borrow(&some).left.clone())
                    + Solution::count_nodes(RefCell::borrow(&some).right.clone())
            }
        } else {
            0
        }
    }

    // 96. Unique Binary Search Trees.
    // https://leetcode.com/problems/unique-binary-search-trees/
    pub fn num_trees(n: i32) -> i32 {
        // To understand the algorithm, learn what the Catalan numbers are.
        let mut c_k: u64 = 1;
        for k in 0..n as u64 {
            c_k = c_k * 2 * (2 * k + 1) / (k + 2);
        }
        c_k as i32
    }

    // 287. Find the Duplicate Number.
    // https://leetcode.com/problems/find-the-duplicate-number/.
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut slow = nums[nums[0] as usize] as usize;
        let mut fast = nums[slow] as usize;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
        }
        fast = nums[0] as usize;
        while slow != fast {
            fast = nums[fast] as usize;
            slow = nums[slow] as usize;
        }
        fast as i32
    }

    // 129. Sum Root to Leaf Numbers.
    // https://leetcode.com/problems/sum-root-to-leaf-numbers/
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nums: Vec<i32> = Vec::new();
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut num: i32, nums: &mut Vec<i32>) {
            if let Some(some) = root {
                num = num * 10 + RefCell::borrow(&some).val;
                let left = RefCell::borrow(&some).left.clone();
                let right = RefCell::borrow(&some).right.clone();
                if left.is_none() && right.is_none() {
                    nums.push(num);
                } else {
                    if left.is_some() {
                        dfs(left, num, nums);
                    }
                    if right.is_some() {
                        dfs(right, num, nums);
                    }
                }
            }
        }
        dfs(root, 0, &mut nums);
        nums.into_iter().sum()
    }

    // 279. Perfect Squares.
    // https://leetcode.com/problems/perfect-squares/
    pub fn num_squares(n: i32) -> i32 {
        if n <= 3 {
            return n;
        }
        let mut dp: Vec<usize> = Vec::with_capacity(n as usize + 1);
        dp.push(0);
        dp.push(1);
        dp.push(2);
        dp.push(3);
        for i in 4..=n as usize {
            dp.push(i);
            let mut j = 1;
            loop {
                let pn = j * j;
                if pn > i {
                    break;
                }
                dp[i] = dp[i].min(1 + dp[i - pn]);
                j += 1;
            }
        }
        *dp.last().unwrap() as i32
    }

    // 332. Reconstruct Itinerary.
    // https://leetcode.com/problems/reconstruct-itinerary/
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        if tickets.is_empty() {
            return vec![];
        }
        let len = tickets.len() + 1;
        let mut adj_map: HashMap<String, Vec<String>> = HashMap::with_capacity(len);
        for tk in tickets {
            let list = adj_map.entry(tk[0].clone()).or_insert(Vec::new());
            list.push(tk[1].clone());
            list.sort_unstable();
            adj_map.entry(tk[1].clone()).or_insert(Vec::new());
        }
        let mut stack: Vec<String> = Vec::with_capacity(len);
        let mut res: Vec<String> = Vec::with_capacity(len);
        stack.push("JFK".to_string());
        while !stack.is_empty() {
            let airport = stack.last().unwrap();
            let list = adj_map.get_mut(airport).unwrap();
            if list.is_empty() {
                res.push(airport.clone());
                stack.pop();
            } else {
                stack.push(list[0].clone());
                list.remove(0);
            }
        }
        res.reverse();
        res
    }

    // 62. Unique Paths.
    // https://leetcode.com/problems/unique-paths/
    pub fn unique_paths(cols: i32, rows: i32) -> i32 {
        let mut table: Vec<Vec<i32>> = vec![vec![1i32; cols as usize]; rows as usize];
        for r in 1..rows as usize {
            for c in 1..cols as usize {
                table[r][c] = table[r - 1][c] + table[r][c - 1]
            }
        }
        table[rows as usize - 1][cols as usize - 1]
    }

    // 212. Word Search II.
    // https://leetcode.com/problems/word-search-ii/
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        #[derive(Clone, Debug, Default)]
        pub struct TrieNode {
            children: HashMap<char, TrieNode>,
            terminal: bool,
        }

        impl TrieNode {
            pub fn new() -> Self {
                TrieNode::default()
            }

            pub fn insert(&mut self, word: &[char]) {
                if word.is_empty() {
                    self.terminal = true;
                    return;
                }

                self.children
                    .entry(word[0])
                    .or_insert(TrieNode::default())
                    .insert(&word[1..]);
            }
        }

        fn dfs(
            board: &mut Vec<Vec<char>>,
            mut num_words: usize,
            node: &mut TrieNode,
            r: isize,
            c: isize,
            path: &mut Vec<char>,
            res: &mut Vec<String>,
        ) {
            if num_words == 0 {
                return;
            }

            if node.terminal {
                res.push(path.iter().collect::<String>());
                node.terminal = false;
                num_words -= 1;
            }

            if r < 0 || c < 0 || r as usize >= board.len() || c as usize >= board[0].len() {
                return;
            }

            let ur = r as usize;
            let uc = c as usize;
            let ch = board[ur][uc];
            if !node.children.contains_key(&ch) {
                return;
            }

            board[ur][uc] = '#';
            path.push(ch);
            for (x, y) in vec![(0, -1), (0, 1), (1, 0), (-1, 0)] {
                dfs(
                    board,
                    num_words,
                    node.children.get_mut(&ch).unwrap(),
                    r + y,
                    c + x,
                    path,
                    res,
                );
            }
            path.pop();
            board[ur][uc] = ch;
        }

        let mut trie = TrieNode::new();
        let num_words = words.len();
        let mut ch_words: Vec<Vec<char>> = Vec::with_capacity(num_words);
        for w in words {
            ch_words.push(w.chars().collect::<Vec<char>>());
            trie.insert(ch_words.last().unwrap());
        }

        let mut res: Vec<String> = Vec::with_capacity(num_words);
        for r in 0..board.len() as isize {
            for c in 0..board[0].len() as isize {
                dfs(
                    &mut board,
                    num_words,
                    &mut trie,
                    r,
                    c,
                    &mut Vec::<char>::new(),
                    &mut res,
                );
            }
        }
        res
    }
}
