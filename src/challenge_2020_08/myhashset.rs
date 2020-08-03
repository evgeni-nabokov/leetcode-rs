// 705. Design HashSet.
// https://leetcode.com/problems/design-hashset/

const TABLE_SIZE: usize = 1009; // Prime number.
const LIST_CAPACITY: usize = 100;

pub struct MyHashSet {
    table: Vec<Option<Vec<i32>>>,
    sorted: Vec<bool>,
}

impl MyHashSet {
    pub fn new() -> Self {
        MyHashSet {
            table: vec![None; TABLE_SIZE],
            sorted: vec![false; TABLE_SIZE],
        }
    }

    pub fn add(&mut self, key: i32) {
        let i = MyHashSet::get_index(key);
        self.sort_list_if_necessary(i);
        if let Some(list) = &mut self.table[i] {
            if list.binary_search(&key).is_err() {
                list.push(key);
                self.sorted[i] = false;
            }
        } else {
            let mut list: Vec<i32> = Vec::with_capacity(LIST_CAPACITY);
            list.push(key);
            self.table[i] = Some(list);
            self.sorted[i] = true;
        }
    }

    pub fn remove(&mut self, key: i32) {
        let i = MyHashSet::get_index(key);
        self.sort_list_if_necessary(i);
        if let Some(list) = &mut self.table[i] {
            if let Ok(j) = list.binary_search(&key) {
                list.remove(j);
            }
        }
    }

    pub fn contains(&mut self, key: i32) -> bool {
        let i = MyHashSet::get_index(key);
        self.sort_list_if_necessary(i);
        if let Some(list) = &mut self.table[i] {
            list.binary_search(&key).is_ok()
        } else {
            false
        }
    }

    #[inline(always)]
    fn sort_list_if_necessary(&mut self, i: usize) {
        if self.sorted[i] { return; }
        if let Some(list) = &mut self.table[i] {
            list.sort_unstable();
            self.sorted[i] = true;
        }
    }

    #[inline(always)]
    fn get_index(key: i32) -> usize {
        key.abs() as usize % TABLE_SIZE
    }
}