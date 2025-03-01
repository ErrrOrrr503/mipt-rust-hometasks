#![forbid(unsafe_code)]

use std::{borrow::Borrow, iter::FromIterator, ops::Index};

////////////////////////////////////////////////////////////////////////////////

// TODO: optimize. It's very dummy - add binsearch and sorting-needed.

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FlatMap<K, V>(Vec<(K, V)>);

impl<K: Ord, V> FlatMap<K, V> {
    pub fn new() -> Self {
        Self(Vec::<(K, V)>::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        self.0.as_slice()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        /*for (k, v) in self.0.iter_mut() {
            if *k == key {
                return Some(std::mem::replace(v, value))
            }
        }
        self.0.push((key, value));
        None*/
        match self.0.iter_mut().find(|(k, _)| *(k.borrow()) == key) {
            Some((_, old_val)) => Some(std::mem::replace(old_val, value)),
            None => {
                self.0.push((key, value));
                self.0.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));
                None
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.iter().find(|(k, _)| k.borrow() == key).map(|(_, v)| v)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.iter().position(|(k, _)| k.borrow() == key).map(|i| self.0.remove(i).1)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.iter().position(|(k, _)| k.borrow() == key).map(|i| self.0.remove(i))
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<K: Ord, Q, V> Index<&Q> for FlatMap<K, V>
where
    K: Borrow<Q>,
    Q: Ord + ?Sized + std::fmt::Debug
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output
    {
        self.get(key).expect(format!("key {:?} is not found", key).as_str())
    }
}

impl<K: Ord, V> Extend<(K, V)> for FlatMap<K, V>
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>
    {
        for (key, value) in iter.into_iter() {
            self.insert(key, value);
        }
    }
}

impl<K: Ord, V> From<Vec<(K, V)>> for FlatMap<K, V>
{
    fn from(vec: Vec<(K, V)>) -> Self
    {
        let mut flatmap = Self::new();
        for (key, value) in vec.into_iter() {
            flatmap.insert(key, value);
        }
        return flatmap
    }
}

impl<K: Ord, V> From<FlatMap<K, V>> for Vec<(K, V)>
{
    fn from(flatmap: FlatMap<K, V>) -> Self
    {
        return flatmap.0
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for FlatMap<K, V>
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>
    {
        let mut flatmap = Self::new();
        for (key, value) in iter.into_iter() {
            flatmap.insert(key, value);
        }
        return flatmap
    }
}

impl<K: Ord, V> IntoIterator for FlatMap<K, V>
{
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    // Required method
    fn into_iter(self) -> Self::IntoIter
    {
        return self.0.into_iter()
    }
}

