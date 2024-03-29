mod recent_counter;
mod two_sum;

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, HashSet, HashMap};
use std::cmp::{max, min, Ordering};
use std::mem::swap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;
use crate::common::list_node::ListNode;

struct Solution;

impl Solution {
    // 624. Maximum Distance in Arrays.
    // https://leetcode.com/problems/maximum-distance-in-arrays/
    // Single-scan solution.
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        // Even though the first array has the min and the max, its range (max - min) is never assigned to res.
        let mut res = 0;
        let (mut min_a, mut max_b) = (arrays[0][0], arrays[0][arrays[0].len() - 1]);
        for i in 1..arrays.len() {
            let (a, b) = (arrays[i][0], arrays[i][arrays[i].len() - 1]);
            res = max(res, max((min_a - b).abs(), (a - max_b).abs()));
            min_a = min(min_a, a);
            max_b = max(max_b, b);
        }
        res
    }

    // 532. K-diff Pairs in an Array.
    // https://leetcode.com/problems/k-diff-pairs-in-an-array/
    // Sorting solution.
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut count = 0;
        let mut left = 0;
        let mut right = 1;
        while left < nums.len() && right < nums.len() {
            if left == right || nums[right] - nums[left] < k {
                right += 1;
            } else if nums[right] - nums[left] > k {
                left += 1;
            } else {
                count += 1;
                left += 1;
                while left < nums.len() && nums[left] == nums[left - 1] {
                    left += 1;
                }
            }
        }
        count
    }

    // 1288. Remove Covered Intervals.
    // https://leetcode.com/problems/remove-covered-intervals/
    // Sorting solution.
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| match a[0].cmp(&b[0]) {
            Ordering::Equal => b[1].cmp(&a[1]),
            x => x
        });
        let mut res = 1;
        let mut a = intervals[0][0];
        let mut b = intervals[0][1];
        for i in 1..intervals.len() {
            let c = intervals[i][0];
            let d = intervals[i][1];
            if !(c >= a && d <= b) {
                a = intervals[i][0];
                b = intervals[i][1];
                res += 1;
            }
        }
        res
    }

    // 701. Insert into a Binary Search Tree.
    // https://leetcode.com/problems/insert-into-a-binary-search-tree/
    pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn new_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: None,
            })))
        }

        if root.is_none() {
            return new_node(val);
        }

        fn dfs(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
            if val > node.as_ref().unwrap().borrow().val {
                if node.as_ref().unwrap().borrow().right.is_none() {
                    node.as_mut().unwrap().borrow_mut().right = new_node(val);
                } else {
                    dfs(&mut node.as_mut().unwrap().borrow_mut().right, val);
                }
            } else {
                if node.as_ref().unwrap().borrow().left.is_none() {
                    node.as_mut().unwrap().borrow_mut().left = new_node(val);
                } else {
                    dfs(&mut node.as_mut().unwrap().borrow_mut().left, val);
                }
            }
        }

        dfs(&mut root, val);
        root
    }

    // 61. Rotate List.
    // https://leetcode.com/problems/rotate-list/
    pub fn rotate_right(head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }

        fn get_length(head: &Option<Box<ListNode>>) -> usize {
            let mut res = 0;
            let mut curr_node = head;
            while curr_node.is_some() {
                curr_node = &curr_node.as_ref().unwrap().next;
                res += 1;
            }
            res
        }

        let len = get_length(&head);
        if len == 1 { return head; }
        k %= len as i32;
        if k == 0 { return head; }

        fn split(mut head: Option<Box<ListNode>>, len: usize) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
            let mut curr_node = &mut head;
            for _ in 0..len {
                let curr_node_inner = curr_node.as_mut().unwrap();
                curr_node = &mut curr_node_inner.next;
            }
            let new_head = curr_node.take();
            (head, new_head)
        }

        fn concat(mut head_1: Option<Box<ListNode>>, head_2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut curr_node = &mut head_1;
            while curr_node.as_ref().unwrap().next.is_some() {
                curr_node = &mut curr_node.as_mut().unwrap().next;
            }

            curr_node.as_mut().unwrap().next = head_2;
            head_1
        }

        let (head_1, head_2) = split(head, len - k as usize);
        concat(head_2, head_1)
    }

    // 704. Binary Search.
    // https://leetcode.com/problems/binary-search/
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => { return mid; },
                Ordering::Greater => { right = mid - 1; }
                Ordering::Less => { left = mid + 1; }
            }
        }
        -1
    }

    // 452. Minimum Number of Arrows to Burst Balloons.
    // https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 2 { return points.len() as i32; }
        points.sort_unstable_by_key(|x| x[1]);
        let mut res = 1;
        let mut b = points[0][1];
        for i in 1..points.len() {
            if points[i][0] > b {
                b = points[i][1];
                res += 1;
            }
        }
        res
    }

    // 316. Remove Duplicate Letters.
    // https://leetcode.com/problems/remove-duplicate-letters/
    pub fn remove_duplicate_letters(s: String) -> String {
        fn solve(chars: Vec<char>) -> Vec<char> {
            if chars.is_empty() { return chars; }
            let mut cnt = [0usize; 26];
            let mut pos = 0;
            for c in &chars {
                cnt[*c as usize - 97] += 1;
            }
            for i in 0..chars.len() {
                if chars[i] < chars[pos] {
                    pos = i;
                }
                let ci = chars[i] as usize - 97;
                cnt[ci] -= 1;
                if cnt[ci] == 0 {
                    break;
                }
            }
            let left_most_char = chars[pos];
            let mut res: Vec<char> = Vec::new();
            res.push(chars[pos]);
            res.extend(solve(chars
                .into_iter()
                .skip(pos + 1)
                .filter(|x| *x != left_most_char)
                .collect()));
            res
        }

        solve(s.chars().collect()).into_iter().collect()
    }

    // 859. Buddy
    // Strings.
    // https://leetcode.com/problems/buddy-strings/
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() || (a.is_empty() && b.is_empty()) { return false; }
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let mut j = 0;
        let mut diff_cnt = 0;
        let mut char_cnt = [0usize; 26];
        let mut has_dup = false;
        for i in 0..a_chars.len() {
            let c_idx = a_chars[i] as usize - 97;
            char_cnt[c_idx] += 1;
            has_dup = has_dup || char_cnt[c_idx] > 1;
            if a_chars[i] != b_chars[i] {
                diff_cnt += 1;
                match diff_cnt {
                    1 => j = i,
                    2 if a_chars[i] == b_chars[j] && a_chars[j] == b_chars[i] => (),
                    _ => return false
                }
            }
        }
        diff_cnt == 2 || (diff_cnt == 0 && has_dup)
    }

    // 213. House Robber II.
    // https://leetcode.com/problems/house-robber-ii/
    // DP solution with O(N) time and O(1) space.
    pub fn rob_ii(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 0; }
        if nums.len() == 1 { return nums[0]; }
        if nums.len() == 2 { return max(nums[0], nums[1]); }

        let mut dp_1: Vec<i32> = vec![0; 2];
        dp_1[0] = nums[0];
        dp_1[1] = max(nums[0], nums[1]);

        let mut dp_2: Vec<i32> = vec![0; 2];
        dp_2[0] = nums[1];
        dp_2[1] = max(nums[1], nums[2]);

        for i in 3..nums.len() {
            let mut tmp = max(dp_1[0] + nums[i - 1], dp_1[1]);
            dp_1[0] = dp_1[1];
            dp_1[1] = tmp;

            tmp = max(dp_2[0] + nums[i], dp_2[1]);
            dp_2[0] = dp_2[1];
            dp_2[1] = tmp;
        }

        max(dp_1.pop().unwrap(), dp_2.pop().unwrap())
    }

    // 253. Meeting Rooms II.
    // https://leetcode.com/problems/meeting-rooms-ii/.
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable();
        let mut allocated_rooms = 0;
        let mut free_rooms = 0;
        let mut end_times: BTreeMap<i32, usize> = BTreeMap::new();
        for i in 0..intervals.len() {
            let start = intervals[i][0];
            let end = intervals[i][1];
            while !end_times.is_empty() {
                if let Some((end, count)) = end_times.iter().next().map(|(k, v)| (*k, *v)) {
                    if end <= start {
                        end_times.remove(&end);
                        free_rooms += count;
                    } else {
                        break;
                    }
                }
            }
            if free_rooms == 0 {
                allocated_rooms += 1;
            } else {
                free_rooms -= 1;
            }
            *end_times.entry(end).or_insert(0) += 1;
        }
        allocated_rooms
    }

    // 189. Rotate Array.
    // https://leetcode.com/problems/rotate-array/
    // Brute force solution (rotation by 1) with (N + k) time and O(1) space.
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.len() < 2 { return; }
        let l = nums.len();
        let k = k as usize % l;
        if k == 0 { return; }
        for _ in 0..k {
            let mut tmp = nums[l - 1];
            for i in 0..l - 1 {
                let new_tmp = nums[i];
                nums[i] = tmp;
                tmp = new_tmp;
            }
            nums[l - 1] = tmp;
        }
    }

    // Copy solution with O(N) time and O(N) space.
    pub fn rotate_v2(nums: &mut Vec<i32>, k: i32) {
        if nums.len() < 2 { return; }
        let l = nums.len();
        let k = k as usize % l;
        if k == 0 { return; }
        let mut new_nums: Vec<i32> = Vec::with_capacity(l);
        new_nums.extend_from_slice(&nums[l - k..]);
        new_nums.extend_from_slice(&nums[..l - k]);
        swap(nums, &mut new_nums);
    }

    // Juggling solution with O(N) time and O(1) space.
    pub fn rotate_v3(nums: &mut Vec<i32>, k: i32) {
        if nums.len() < 2 { return; }
        let l = nums.len();
        let k = k as usize % l;
        if k == 0 { return; }

        fn get_gcd(a: usize, b: usize) -> usize {
            if b == 0 { a } else { get_gcd(b, a % b) }
        }

        for i in 0..get_gcd(l, k) {
            let mut j = i;
            let mut tmp = nums[j];
            loop {
                let new_j = (j + k) % l;
                let new_tmp = nums[new_j];
                nums[new_j] = tmp;
                tmp = new_tmp;
                j = new_j;
                // We came back to the start index.
                if j == i { break; }
            }
        }
    }

    // Reverse solution with O(N) time and O(1) space.
    pub fn rotate_v4(nums: &mut Vec<i32>, k: i32) {
        if nums.len() < 2 { return; }
        let l = nums.len();
        let k = k as usize % l;
        if k == 0 { return; }

        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }

    // 74. Search a 2D Matrix.
    // https://leetcode.com/problems/search-a-2d-matrix/
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() { return false; }
        match matrix.binary_search_by_key(&target, |x| x[0]) {
            Err(row) if row == 0 => false,
            Err(row) => match matrix[row - 1].binary_search(&target) {
                Err(_) => false,
                Ok(_) => true
            }
            Ok(_) => true
        }
    }

    // 187. Repeated DNA Sequences.
    // https://leetcode.com/problems/repeated-dna-sequences/
    // Rabin-Karp solution.
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let seq_len = 10;
        if s.len() <= seq_len { return vec![]; }

        let base = 4;
        let modulus = 4_294_967_296;
        // Rolling hash.
        let mut hash= 0u64;
        // Const value to be used often: (base ^ len) % modulus.
        let mut base_pow_len = 1u64;
        // Compute the hash of first len codes and base_pow_len.
        let codes: Vec<_> = s.chars().map(|x| match x {
            'A' => 0, 'C' => 1, 'G' => 2, 'T' => 3, _ => unreachable!()
        }).collect();
        for i in 0..seq_len {
            hash = (hash * base + codes[i]) % modulus;
            base_pow_len = (base_pow_len * base) % modulus;
        }
        let hash_num = codes.len() - seq_len;
        let mut seen_hashes: HashSet<u64> = HashSet::with_capacity(hash_num);
        let mut repeated_seqs: HashSet<&[u64]> = HashSet::new();
        seen_hashes.insert(hash);
        for start in 1..=hash_num {
            // Compute rolling hash in O(1) time.
            hash = (hash * base - codes[start - 1] * base_pow_len % modulus + modulus) % modulus;
            hash = (hash + (codes[start + seq_len - 1])) % modulus;
            if seen_hashes.contains(&hash) {
                repeated_seqs.insert(&codes[start..start + seq_len]);
            } else {
                seen_hashes.insert(hash);
            }
        }
        repeated_seqs.into_iter().map(|x| x.into_iter().map(|y| match y {
            0 => 'A', 1 => 'C', 2 => 'G', 3 => 'T', _ => unreachable!()
        }).collect()).collect()
    }

    // 735. Asteroid Collision.
    // https://leetcode.com/problems/asteroid-collision/
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(asteroids.len());
        for a in asteroids {
            if a > 0 || res.is_empty() {
                res.push(a);
                continue;
            }
            while !res.is_empty() {
                let last = res[res.len() - 1];
                if last < 0 {
                    res.push(a);
                    break;
                } else {
                    match a.abs().cmp(&last) {
                        Ordering::Equal => {
                            res.pop();
                            break;
                        },
                        Ordering::Greater => { res.pop(); },
                        Ordering::Less => break,
                    }
                    if res.is_empty() {
                        res.push(a);
                        break;
                    }
                }
            }
        }
        res
    }

    // 111. Minimum Depth of Binary Tree.
    // https://leetcode.com/problems/minimum-depth-of-binary-tree/
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut count: i32) -> i32 {
            if let Some(node_inner) = node {
                count += 1;
                match (node_inner.borrow().left.is_some(), node_inner.borrow().right.is_some()) {
                    (true, true) => min(dfs(&node_inner.borrow().left, count), dfs(&node_inner.borrow().right, count)),
                    (true, false) => dfs(&node_inner.borrow().left, count),
                    (false, true) => dfs(&node_inner.borrow().right, count),
                    (false ,false) => count
                }
            } else {
                count
            }
        }
        dfs(&root, 0)
    }

    // 1510. Stone Game IV.
    // https://leetcode.com/problems/stone-game-iv/
    // DFS solution.
    pub fn winner_square_game(n: i32) -> bool {
        fn dfs(n: i32, memo: &mut HashMap<i32, bool>) -> bool {
            match memo.get(&n) {
                Some(b) => return *b,
                None => {
                    let sr = (n as f64).sqrt() as i32;
                    for i in 1..=sr {
                        if !dfs(n - i * i, memo) {
                            memo.insert(n, true);
                            return true;
                        }
                    }
                }
            }
            memo.insert(n, false);
            false
        }

        let mut memo: HashMap<i32, bool> = HashMap::new();
        memo.insert(0, false);
        dfs(n, &mut memo)
    }

    // DP solution #1.
    pub fn winner_square_game_v2(n: i32) -> bool {
        let mut dp: Vec<bool> = vec![false; n as usize + 1];
        for i in 0..=n as usize {
            let sr = (i as f64).sqrt() as usize;
            for k in 1..=sr {
                if dp[i - k * k] == false {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n as usize]
    }

    // DP solution #2.
    pub fn winner_square_game_v3(n: i32) -> bool {
        let mut dp: Vec<bool> = vec![false; n as usize + 1];
        for i in 0..=n as usize {
            if dp[i] { continue; }
            let mut k = 1;
            while i + k * k <= n as usize {
                dp[i + k * k] = true;
                k += 1;
            }
        }
        dp[n as usize]
    }

    // 228. Summary Ranges
    // https://leetcode.com/problems/summary-ranges/
    // Solution with O(N) time and O(1) space.
    pub fn summary_ranges(mut nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() { return vec![]; };
        let mut res: Vec<String> = Vec::new();
        let mut start = nums[0];
        // Pad the right.
        nums.push(i32::MAX);
        for i in 1..nums.len() {
            if nums[i - 1] != nums[i] - 1 {
                res.push(if start == nums[i - 1] { format!("{}", start) } else { format!("{}->{}", start, nums[i - 1]) });
                start = nums[i];
            }
        }
        res
    }

    // 849. Maximize Distance to Closest Person.
    // https://leetcode.com/problems/maximize-distance-to-closest-person/
    // Solution with O(N) time and O(1) space.
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut left_empty_seats = 0;
        let mut right_empty_seats = 0;
        let mut max_empty_seats = 0;
        let mut curr_empty_seats = 0;
        let mut l = 0;
        while seats[l] == 0 {
            left_empty_seats += 1;
            l += 1;
        }
        if left_empty_seats >= seats.len() / 2 {
            return left_empty_seats as i32
        }
        let mut r = seats.len() - 1;
        while seats[r] == 0 {
            right_empty_seats += 1;
            r -= 1
        }
        if left_empty_seats + right_empty_seats >= seats.len() / 2 {
            return max(left_empty_seats, right_empty_seats) as i32
        }
        for i in l + 1..=r {
            if seats[i] == 1 {
                max_empty_seats = max(max_empty_seats, curr_empty_seats);
                curr_empty_seats = 0;
            } else {
                curr_empty_seats += 1;
            }
        }
        max((max_empty_seats - 1) / 2 + 1, max(left_empty_seats, right_empty_seats)) as i32
    }
}