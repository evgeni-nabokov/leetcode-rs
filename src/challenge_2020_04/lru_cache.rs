// 146. LRU Cache.
// https://leetcode.com/problems/lru-cache/

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::common::doubly_linked_list::{DoublyLinkedList, DoublyLinkedListNode};

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<DoublyLinkedListNode<(i32, i32)>>>>,
    list: DoublyLinkedList<(i32, i32)>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::with_capacity(capacity as usize),
            list: DoublyLinkedList::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.map.entry(key) {
            Entry::Occupied(mut o) => {
                let (k, v) = self.list.remove(o.get().clone());
                *o.get_mut() = self.list.push_front((k, v));
                v
            }
            _ => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.list.remove(Rc::clone(self.map.get(&key).unwrap()));
        } else if self.capacity == self.list.len() {
            if let Some((k, _v)) = self.list.pop_back() {
                self.map.remove(&k);
            }
        }
        self.map.insert(key, self.list.push_front((key, value)));
    }
}
