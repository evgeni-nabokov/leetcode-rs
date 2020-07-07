use crate::common::doubly_linked_list::{DoublyLinkedList, DoublyLinkedListNode};

#[test]
fn list_push_front_test() {
    let mut list = DoublyLinkedList::new();
    list.push_front(1);
    assert_eq!(list.to_vec(), vec![1]);
    assert_eq!(list.to_vec_backward(), vec![1]);
    assert_eq!(list.len(), 1);
    list.push_front(2);
    assert_eq!(list.to_vec(), vec![2, 1]);
    assert_eq!(list.to_vec_backward(), vec![1, 2]);
    assert_eq!(list.len(), 2);
}

#[test]
fn list_push_back_test() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    assert_eq!(list.to_vec(), vec![1]);
    assert_eq!(list.to_vec_backward(), vec![1]);
    assert_eq!(list.len(), 1);
    list.push_back(2);
    assert_eq!(list.to_vec(), vec![1, 2]);
    assert_eq!(list.to_vec_backward(), vec![2, 1]);
    assert_eq!(list.len(), 2);
}

#[test]
fn list_pop_front_test() {
    let mut list = DoublyLinkedList::new();
    list.push_front(1);
    list.push_back(2);
    assert_eq!(list.to_vec(), vec![1, 2]);
    assert_eq!(list.to_vec_backward(), vec![2, 1]);
    assert_eq!(list.len(), 2);

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.to_vec(), vec![2]);
    assert_eq!(list.to_vec_backward(), vec![2]);
    assert_eq!(list.len(), 1);

    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.to_vec(), vec![]);
    assert_eq!(list.to_vec_backward(), vec![]);
    assert_eq!(list.len(), 0);

    assert_eq!(list.pop_front(), None);
    assert_eq!(list.to_vec(), vec![]);
    assert_eq!(list.to_vec_backward(), vec![]);
    assert_eq!(list.len(), 0);
}

#[test]
fn list_pop_back_test() {
    let mut list = DoublyLinkedList::new();
    list.push_front(1);
    list.push_back(2);
    assert_eq!(list.to_vec(), vec![1, 2]);
    assert_eq!(list.to_vec_backward(), vec![2, 1]);
    assert_eq!(list.len(), 2);

    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.to_vec(), vec![1]);
    assert_eq!(list.to_vec_backward(), vec![1]);
    assert_eq!(list.len(), 1);

    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.to_vec(), vec![]);
    assert_eq!(list.to_vec_backward(), vec![]);
    assert_eq!(list.len(), 0);

    assert_eq!(list.pop_back(), None);
    assert_eq!(list.to_vec(), vec![]);
    assert_eq!(list.to_vec_backward(), vec![]);
    assert_eq!(list.len(), 0);
}

#[test]
fn list_remove_test() {
    let mut list = DoublyLinkedList::new();
    let node_1 = list.push_front(1);
    let node_2 = list.push_back(2);
    assert_eq!(list.to_vec(), vec![1, 2]);
    assert_eq!(list.to_vec_backward(), vec![2, 1]);
    assert_eq!(list.len(), 2);

    assert_eq!(list.remove(node_1), 1);
    assert_eq!(list.to_vec(), vec![2]);
    assert_eq!(list.to_vec_backward(), vec![2]);
    assert_eq!(list.len(), 1);

    assert_eq!(list.remove(node_2.clone()), 2);
    assert_eq!(list.to_vec(), vec![]);
    assert_eq!(list.to_vec_backward(), vec![]);
    assert_eq!(list.len(), 0);

    assert_eq!(list.remove(node_2), 2);
    assert_eq!(list.to_vec(), vec![]);
    assert_eq!(list.to_vec_backward(), vec![]);
    assert_eq!(list.len(), 0);
}

#[test]
fn new_node_test() {
    let val = 1;
    let node = DoublyLinkedListNode::new(val);
    assert_eq!(node.borrow().val, val);
    assert_eq!(node.borrow().prev, None);
    assert_eq!(node.borrow().next, None);
}