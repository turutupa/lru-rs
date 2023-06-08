use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
struct LRU<K, V> {
    capacity: u32,
    length: u32,
    head: Option<Box<Node<V>>>,
    tail: Option<Box<Node<V>>>,
    lookup: HashMap<K, Node<V>>,
    reverse_lookup: HashMap<Node<V>, K>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Node<V> {
    next: Option<Box<Node<V>>>,
    prev: Option<Box<Node<V>>>,
    value: V,
}

impl<V> Node<V> {
    pub fn set(&mut self, value: V) {
        self.value = value;
    }
}

fn create_node<V>(value: V) -> Node<V> {
    return Node {
        next: None,
        prev: None,
        value,
    };
}

trait LeastRecentlyUsed<K, V> {
    fn new(capacity: u32) -> Self;

    fn update(&mut self, key: K, value: V);

    fn get(&mut self, key: &K) -> Option<&Node<V>>;
}

impl<K, V> LeastRecentlyUsed<K, V> for LRU<K, V>
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
        match self.lookup.get_mut(&key) {
            Some(node) => {
                node.set(value);
                self.get(&key);
            }
            None => {
                let node = create_node(value);
                self.prepend(&node);
                self.lookup.insert(key, node);
                self.length += 1;
                self.trim_cache();
            }
        }
    }

    fn get(&mut self, key: &K) -> Option<&Node<V>> {
        match self.lookup.get(key) {
            // immutability borrow
            Some(node) => {
                self.detach(node); // mutability borrow 1
                self.prepend(node); // mutability borrow 2
                Some(node)
            }
            None => None,
        }
    }
}

impl<K, V> LRU<K, V>
where
    K: Eq + Hash,
{
    fn detach(&mut self, node: &Node<V>) {
        // if let Some(next) = node.next.as_ref().map(|boxed_node| &**boxed_node) {}

        // remove prev links
        // remove next links
        // update head/tail if necessary
    }

    fn prepend(&self, node: &Node<V>) {
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
