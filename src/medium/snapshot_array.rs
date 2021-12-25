// 1146. Snapshot Array.
// https://leetcode.com/problems/snapshot-array/
// Time complexity:
//      new(): O(L), L - SnapshotArray's length.
//      set(): O(1).
//      snap(): O(1).
//      get(): O(LogS), S - number of snapshots.
// Space complexity: O(L x S).
//      new(): O(L), L - SnapshotArray's length.
//      set(): O(1).
//      snap(): O(1).
//      get(): O(1).

pub struct SnapshotArray {
    snapshot: i32,
    container: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
    pub fn new(length: i32) -> Self {
        SnapshotArray {
            snapshot: 0,
            container: vec![vec![]; length as usize],
        }
    }

    pub fn set(&mut self, index: i32, val: i32) {
        let i = index as usize;
        if self.container[i].is_empty() {
            self.container[i].push((self.snapshot, val));
        } else {
            let s = self.container[i].len() - 1;
            if self.container[i][s].0 != self.snapshot {
                self.container[i].push((self.snapshot, val));
            } else {
                self.container[i][s].1 = val;
            }
        }
    }

    pub fn snap(&mut self) -> i32 {
        self.snapshot += 1;
        self.snapshot - 1
    }

    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        let i = index as usize;
        match self.container[i].binary_search_by_key(&snap_id, |x| x.0) {
            Ok(snap_idx) => self.container[i][snap_idx].1,
            Err(s) => {
                if s == 0 {
                    0
                } else {
                    self.container[i][s - 1].1
                }
            }
        }
    }
}
