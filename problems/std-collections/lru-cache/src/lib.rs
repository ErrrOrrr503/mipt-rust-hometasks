#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    cur_time: usize,
    len: usize,
    capacity: usize,
    map: HashMap<K, (V, usize)>,
    usage_times: BTreeMap<usize, K>,
}

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be positive");
        Self {
            cur_time: 0,
            len: 0,
            capacity: capacity,
            map: HashMap::with_capacity(capacity),
            usage_times: BTreeMap::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(entry) = self.map.get_mut(&key) {
            self.cur_time += 1;
            let old_time = std::mem::replace(&mut entry.1, self.cur_time);
            self.usage_times.remove(&old_time);
            self.usage_times.insert(self.cur_time, key.clone());
            Some(&entry.0)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.cur_time += 1;
        if let Some(curr_entry) = self.map.get_mut(&key) {
            /* old entry updated */
            let old_val = std::mem::replace(&mut curr_entry.0, value);
            let old_time = std::mem::replace(&mut curr_entry.1, self.cur_time);
            self.usage_times.remove(&old_time);
            self.usage_times.insert(self.cur_time, key);
            Some(old_val)
        } else {
            /* new entry inserted  */
            self.map.insert(key.clone(), (value, self.cur_time));
            if self.len == self.capacity {
                let (_, lru_key) = self.usage_times.pop_first().unwrap();
                self.usage_times.insert(self.cur_time, key);
                self.map.remove(&lru_key);
                None
            } else {
                self.len += 1;
                self.usage_times.insert(self.cur_time, key);
                None
            }
        }
    }
}
