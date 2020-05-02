use std::collections::{HashMap, LinkedList, VecDeque};

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    list: VecDeque<(i32, i32)>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::with_capacity(capacity as usize),
            list: VecDeque::with_capacity(capacity as usize),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key) {
            Some(idx) => {
                let (k, v) = self.list.remove(idx.clone()).unwrap();
                *self.map.get_mut(&k).unwrap() = 0;
                self.list.push_front((k, v));

                println!("map={:?}, list={:?}", self.map, self.list);

                v
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(idx) = self.map.get(&key) {
            self.list.remove(idx.clone());
        } else {
            if self.list.len() == self.capacity {
                let (k, _) = self.list.pop_back().unwrap();
                self.map.remove(&k);
            } else {}
        }
        *self.map.entry(key).or_insert(0) = 0;
        self.list.push_front((key, value));
        println!("map={:?}, list={:?}", self.map, self.list);
    }
}

#[test]
fn lru_cache_test() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}
