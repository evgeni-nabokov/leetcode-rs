use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub struct DoublyLinkedListNode<T: Copy + Debug> {
    pub val: T,
    pub next: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
    pub prev: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
}

impl<T> DoublyLinkedListNode<T>
where T: Copy + Debug {
    #[inline]
    pub fn new(val: T) -> Rc<RefCell<DoublyLinkedListNode<T>>> {
        Rc::new(RefCell::new(DoublyLinkedListNode {
            val,
            next: None,
            prev: None,
        }))
    }
}

#[derive(PartialEq, Debug)]
pub struct DoublyLinkedList<T: Copy + Debug> {
    head: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
    tail: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
    length: usize,
}

impl<T> DoublyLinkedList<T>
where T: Copy + Debug
{
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push_front(&mut self, val: T) -> Rc<RefCell<DoublyLinkedListNode<T>>> {
        let new_head = DoublyLinkedListNode::new(val);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
                new_head.borrow_mut().next = Some(Rc::clone(&old_head));
            },
            _ => {
                self.tail = Some(Rc::clone(&new_head));
            }
        }

        self.head = Some(Rc::clone(&new_head));
        self.length += 1;

        new_head
    }

    pub fn push_back(&mut self, val: T) -> Rc<RefCell<DoublyLinkedListNode<T>>> {
        let new_tail = DoublyLinkedListNode::new(val);

        match &self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(Rc::clone(&old_tail));
            },
            _ => {
                self.head = Some(Rc::clone(&new_tail));
            }
        }

        self.tail = Some(Rc::clone(&new_tail));
        self.length += 1;

        new_tail
    }

    pub fn remove(&mut self, node: Rc<RefCell<DoublyLinkedListNode<T>>>) -> T {
        if node.borrow().prev.is_none() && node.borrow().next.is_none() {
            self.head = None;
            self.tail = None;
            self.length = 0;
            return node.borrow().val
        }

        let mut bn = node.borrow_mut();
        match bn.prev.take() {
            Some(prev_node) => {
                match bn.next.take() {
                    Some(next_node) => {
                        prev_node.borrow_mut().next = Some(Rc::clone(&next_node));
                        next_node.borrow_mut().prev = Some(prev_node);
                    },
                    _ => {
                        // node is the tail.
                        prev_node.borrow_mut().next = None;
                        self.tail = Some(prev_node);
                    }
                }
            }
            _ => {
                // node is the head.
                self.head = bn.next.clone();
                self.head.as_mut().unwrap().borrow_mut().prev = None;
            }
        }
        self.length -= 1;
        bn.val
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head_node) => {
                match head_node.borrow_mut().next.as_mut() {
                    Some(next_node) => {
                        next_node.borrow_mut().prev = None;
                        self.head = Some(Rc::clone(&next_node));
                    }
                    _ => {
                        // node is the tail too.
                        self.tail = None;
                    }
                }
                self.length -= 1;
                Some(head_node.borrow().val)
            },
            _ => None,
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(tail_node) => {
                match tail_node.borrow_mut().prev.as_mut() {
                    Some(prev_node) => {
                        prev_node.borrow_mut().next = None;
                        self.tail = Some(Rc::clone(&prev_node));
                    },
                    _ => {
                        // node is the head too.
                        self.head = None;
                    }
                }
                self.length -= 1;
                Some(tail_node.borrow().val)
            },
            _ => None,
        }
    }

    pub fn front(&self) -> Option<Rc<RefCell<DoublyLinkedListNode<T>>>> {
        match &self.head {
            Some(inner) => Some(inner.clone()),
            _ => None,
        }
    }

    pub fn back(&self) -> Option<Rc<RefCell<DoublyLinkedListNode<T>>>> {
        match &self.tail {
            Some(inner) => Some(inner.clone()),
            _ => None,
        }
    }

    pub fn to_vec(&self) -> Vec<T>{
        let mut res: Vec<T> = Vec::with_capacity(self.length);
        let mut node = self.head.clone();
        while let Some(some) = node {
            res.push(some.borrow().val);
            node = some.borrow().next.clone();
        }
        res
    }

    pub fn to_vec_backward(&self) -> Vec<T>{
        let mut res: Vec<T> = Vec::with_capacity(self.length);
        let mut node = self.tail.clone();
        while let Some(some) = node {
            res.push(some.borrow().val);
            node = some.borrow().prev.clone();
        }
        res
    }
}