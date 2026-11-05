//! LRU cache. First do O(n) with Vec/VecDeque + HashMap, then think about O(1).
//! O(1) in safe Rust needs index-based linked list in a Vec (slab) instead of pointers.
use std::collections::HashMap;
use std::hash::Hash;

pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    // add whatever bookkeeping you need for recency
}

impl<K: Hash + Eq + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self { todo!() }
    pub fn get(&mut self, k: &K) -> Option<&V> { todo!() }
    pub fn put(&mut self, k: K, v: V) { todo!() }
    pub fn len(&self) -> usize { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evicts_least_recently_used() {
        let mut c = LruCache::new(2);
        c.put("a", 1);
        c.put("b", 2);
        assert_eq!(c.get(&"a"), Some(&1)); // a is now most recent
        c.put("c", 3);                     // evicts b
        assert_eq!(c.get(&"b"), None);
        assert_eq!(c.get(&"a"), Some(&1));
        assert_eq!(c.get(&"c"), Some(&3));
        assert_eq!(c.len(), 2);
    }
}
