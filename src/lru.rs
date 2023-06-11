use std::cell::{RefCell, RefMut};
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug)]
struct LRU<K, V: Clone> {
    capacity: u32,
    length: u32,
    head: Option<Rc<RefCell<InternalNode<V>>>>,
    tail: Option<Rc<RefCell<InternalNode<V>>>>,
    lookup: HashMap<K, Rc<RefCell<InternalNode<V>>>>,
    reverse_lookup: HashMap<Rc<RefCell<InternalNode<V>>>, K>,
}

#[derive(Debug, Eq, PartialEq)]
struct InternalNode<V> {
    next: Option<Rc<RefCell<InternalNode<V>>>>,
    prev: Option<Rc<RefCell<InternalNode<V>>>>,
    value: V,
}

impl<V: Clone> InternalNode<V> {
    fn set(&mut self, value: V) {
        self.value = value;
    }

    fn get(&self) -> V {
        return self.value.clone();
    }
}

struct Node<V> {
    node: Rc<RefCell<InternalNode<V>>>,
}

impl<V: Clone> Node<V> {
    pub fn new(node: Rc<RefCell<InternalNode<V>>>) -> Node<V> {
        Node { node }
    }

    pub fn set(&mut self, value: V) {
        self.node.borrow_mut().set(value);
    }

    pub fn get(&self) -> V {
        let borrowed_node = self.node.borrow();
        borrowed_node.get().clone()
    }
}

fn create_node<V>(value: V) -> InternalNode<V> {
    return InternalNode {
        next: None,
        prev: None,
        value,
    };
}

trait LeastRecentlyUsed<K, V> {
    fn new(capacity: u32) -> Self;

    fn update(&mut self, key: K, value: V);

    fn get(&mut self, key: &K) -> Option<Node<V>>;
}

impl<K, V: Clone> LeastRecentlyUsed<K, V> for LRU<K, V>
where
    K: Eq + Hash,
{
    fn new(capacity: u32) -> Self {
        return LRU {
            capacity,
            length: 0,
            head: None,
            tail: None,
            lookup: HashMap::new(),
            reverse_lookup: HashMap::new(),
        };
    }

    fn update(&mut self, key: K, value: V) {
        match self.lookup.get(&key).cloned() {
            Some(node) => {
                node.borrow_mut().set(value);
                self.detach(&mut node.borrow_mut());
                self.prepend(&mut node.borrow_mut());
            }
            None => {
                let node = Rc::new(RefCell::new(create_node(value)));
                self.prepend(&mut node.borrow_mut());
                self.lookup.insert(key, node);
                self.length += 1;
                self.trim_cache();
            }
        }
    }

    fn get(&mut self, key: &K) -> Option<Node<V>> {
        match self.lookup.get(key).cloned() {
            Some(node) => {
                self.detach(&mut node.borrow_mut());
                self.prepend(&mut node.borrow_mut());
                Some(Node::new(node))
            }
            None => None,
        }
    }
}

impl<K, V: Clone> LRU<K, V>
where
    K: Eq + Hash,
{
    fn detach(&mut self, node: &mut RefMut<InternalNode<V>>) {
        let prev_node = node.prev.take();
        let next_node = node.next.take();
        if let Some(prev) = prev_node {
            prev.borrow_mut().next = next_node.clone();
        }
        if let Some(next) = next_node.clone() {
            next.borrow_mut().prev = next_node;
        }

        node.next = None;
        node.prev = None;

        // update head/tail if necessary
    }

    fn prepend(&self, node: &mut RefMut<InternalNode<V>>) {
        // update head (and tail if necessary)
        // update links to prev head
    }

    fn trim_cache(&self) {
        // remove least recently used elements if length greater than capacity
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn lru_new() -> LRU<String, u32> {
        let mut lru: LRU<String, u32> = LRU::new(10);
        for i in 0..11 {
            lru.update(i.to_string(), i);
        }
        return lru;
    }

    #[test]
    fn test_example() {
        let mut lru = lru_new();
        let val = lru.get(&"0".to_string());
        match val {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }
}
