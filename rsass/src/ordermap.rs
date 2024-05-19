use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderMap<K, V>(Vec<(K, V)>);

impl<K: Clone + PartialEq, V: Clone> OrderMap<K, V> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn singleton(key: K, value: V) -> Self {
        Self(vec![(key, value)])
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        for (ref k, ref mut v) in &mut self.0 {
            if k == &key {
                return Some(std::mem::replace(v, value));
            }
        }
        self.0.push((key, value));
        None
    }
    pub fn iter(&self) -> Iter<(K, V)> {
        self.0.iter()
    }
    pub fn keys(&self) -> impl Iterator<Item = &'_ K> {
        self.0.iter().map(|(k, _v)| k)
    }
    pub fn values(&self) -> impl Iterator<Item = &'_ V> {
        self.0.iter().map(|(_k, v)| v)
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        for (k, v) in &self.0 {
            if k == key {
                return Some(v);
            }
        }
        None
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for kv in &mut self.0 {
            if &kv.0 == key {
                return Some(&mut kv.1);
            }
        }
        None
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        for (i, (k, _v)) in self.0.iter().enumerate() {
            if k == key {
                return Some(self.0.remove(i).1);
            }
        }
        None
    }
    pub fn contains_key(&self, key: &K) -> bool {
        self.0.iter().any(|(k, _v)| k == key)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn get_item(&self, i: usize) -> Option<&(K, V)> {
        self.0.get(i)
    }
    pub fn set_item(&mut self, i: usize, key: K, value: V) {
        self.0[i] = (key, value);
    }
}

impl<K, V> Default for OrderMap<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K, V> IntoIterator for OrderMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<(K, V)>;
    fn into_iter(self) -> IntoIter<(K, V)> {
        self.0.into_iter()
    }
}

impl<K, V> FromIterator<(K, V)> for OrderMap<K, V> {
    fn from_iter<T>(i: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        Self(Vec::from_iter(i))
    }
}
