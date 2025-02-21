#![forbid(unsafe_code)]

use crate::node::Node;

use std::borrow::Borrow;

pub struct AVLTreeMap<K, V> {
    head: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        match &self.head {
            Some(head) => head.len(),
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        if let Some((_, rv)) = Node::get_key_value(&self.head, key) {
            return Some(rv)
        }
        None
    }

    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        Node::get_key_value(&self.head, key)
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        Node::get_key_value(&self.head, key).is_some()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (newhead, ret) =  Node::insert(self.head.take(), key, value);
        self.head = Some(newhead);
        return ret
    }

    pub fn nth_key_value(&self, k: usize) -> Option<(&K, &V)> {
        Node::nth_key_value(&self.head, k)
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        match self.remove_entry(key) {
            Some((_key, value)) => Some(value),
            None => None,
        }
    }
    pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        match self.head.take() {
            Some(head) => {
                let (newhead, ret) = Node::remove_entry(head, key);
                self.head = newhead;
                ret
            },
            None => None,
        }
    }
}
