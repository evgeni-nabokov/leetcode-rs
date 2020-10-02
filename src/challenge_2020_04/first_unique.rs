use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::hash_map::Entry;

use crate::common::doubly_linked_list::{DoublyLinkedList, DoublyLinkedListNode};

pub struct FirstUnique {
    queue: DoublyLinkedList<i32>,
    map: HashMap<i32, Option<Rc<RefCell<DoublyLinkedListNode<i32>>>>>
}

impl FirstUnique {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut res = FirstUnique {
            queue: DoublyLinkedList::new(),
            map: HashMap::with_capacity(nums.len() * 2),
        };
        for n in nums {
            res.add(n);
        }
        res
    }

    pub fn show_first_unique(&self) -> i32 {
        if let Some(s) = self.queue.front() {
            s.borrow().val
        } else {
            -1
        }
    }

    pub fn add(&mut self, value: i32) {
        match self.map.entry(value) {
            Entry::Occupied(mut o) => {
                if let Some(inner) = o.get_mut().take() {
                    o.insert(None);
                    self.queue.remove(inner);
                }
            },
            Entry::Vacant(v) => {
                v.insert(Some(self.queue.push_back(value)));
            }
        }
    }
}