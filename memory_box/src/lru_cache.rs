use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

/// LRU Cache Implementation
pub struct LruCache<K, V> {
    map: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    head: Option<Rc<RefCell<Node<K, V>>>>,
    tail: Option<Rc<RefCell<Node<K, V>>>>,
    capacity: usize,
    size: usize,
}

/// Node of the doubly linked list
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Rc<RefCell<Node<K, V>>>>,
    next: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> LruCache<K, V> {
    /// Create a new LRU cache with a given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
            size: 0,
        }
    }

    /// Insert a key-value pair into the cache.
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(node) = self.map.remove(&key) {
            // Key exists, update value and move node to the front
            node.borrow_mut().value = value;
            self.move_to_front(node.clone());
            self.map.insert(key, node);
        } else {
            // Key does not exist, create a new node
            let new_node = Rc::new(RefCell::new(Node {
                key: key.clone(),
                value,
                prev: None,
                next: None,
            }));

            if self.size == self.capacity {
                // Evict the least recently used item
                self.evict();
            } else {
                self.size += 1;
            }

            // Add the new node to the front of the list
            self.add_to_front(new_node.clone());
            self.map.insert(key, new_node);
        }
    }

    /// Get a value associated with a key.
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(node) = self.map.remove(key) {
            self.move_to_front(node.clone());
            let value = node.borrow().value.clone(); // Clone the value
            self.map.insert(key.clone(), node); // Reinsert node
            Some(value)
        } else {
            None
        }
    }

    /// Remove a key-value pair from the cache.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(node) = self.map.remove(key) {
            self.unlink(node.clone());
            self.size -= 1;
            // Handle the case where Rc::try_unwrap fails
            match Rc::try_unwrap(node) {
                Ok(inner) => Some(inner.into_inner().value),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    /// Add a node to the front of the doubly linked list.
    fn add_to_front(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        node.borrow_mut().next = self.head.clone();
        node.borrow_mut().prev = None;

        if let Some(ref head) = self.head {
            head.borrow_mut().prev = Some(node.clone());
        } else {
            self.tail = Some(node.clone());
        }

        self.head = Some(node);
    }

    /// Move an existing node to the front of the list.
    fn move_to_front(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        self.unlink(node.clone());
        self.add_to_front(node);
    }

    /// Unlink a node from the doubly linked list.
    fn unlink(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        let prev = node.borrow_mut().prev.take();
        let next = node.borrow_mut().next.take();

        if let Some(ref prev_node) = prev {
            prev_node.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(ref next_node) = next {
            next_node.borrow_mut().prev = prev.clone();
        } else {
            self.tail = prev.clone();
        }
    }

    /// Evict the least recently used item.
    fn evict(&mut self) {
        if let Some(tail_node) = self.tail.take() {
            let key_to_remove = tail_node.borrow().key.clone();
            self.remove(&key_to_remove);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lru_cache_works() {
        let mut cache = LruCache::new(3);

        // Insert values into the cache
        cache.insert(1, "one");
        cache.insert(2, "two");
        cache.insert(3, "three");

        // Access values
        assert_eq!(cache.get(&2), Some("two"));
        assert_eq!(cache.get(&4), None);

        // Insert another value, causing eviction of key 1
        cache.insert(4, "four");
        assert_eq!(cache.get(&1), None);

        // Verify remaining keys
        assert_eq!(cache.get(&3), Some("three"));
        assert_eq!(cache.get(&4), Some("four"));
    }
}
