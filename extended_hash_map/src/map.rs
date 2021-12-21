use std::{
    collections::HashMap,
    hash::Hash,
    ops::Index,
    borrow::Borrow
};
use super::{
    iloc::Iloc,
    ploc::Ploc
};

#[derive(Debug)]
pub struct ExtendedHashMap<'a, K, V> {
    pub map: HashMap<K, V>,
    pub iloc: Iloc<'a, K, V>,
    pub ploc: Ploc<'a, K, V>
}

impl<'a, K: 'a, V: 'a> ExtendedHashMap<'a, K, V> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<K, V> ExtendedHashMap<'_, K, V> 
where
    K: Eq + Hash
{
    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.map.insert(k, v)
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }
}

impl<K, V> Default for ExtendedHashMap<'_, K, V>
{
    #[inline]
    fn default() -> Self {
        let mut hash_map = Self {
            map: HashMap::<K, V>::default(),
            iloc: Iloc { map: None },
            ploc: Ploc { map: None }
        };

        hash_map.iloc.map = unsafe { 
            Some(std::ptr::NonNull::new_unchecked(&mut hash_map.map).as_ref()) 
        };
        hash_map.ploc.map = unsafe { 
            Some(std::ptr::NonNull::new_unchecked(&mut hash_map.map).as_ref()) 
        };
        hash_map
    }
}

impl<K, Q: ?Sized, V> Index<&Q> for ExtendedHashMap<'_, K, V>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
{
    type Output = V;

    #[inline]
    fn index(&self, key: &Q) -> &V {
        &self.map[key]
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for ExtendedHashMap<'_, K, V>
where
    K: Eq + Hash,
{
    fn from(arr: [(K, V); N]) -> Self {
        let mut hash_map = Self {
            map: HashMap::from(arr),
            iloc: Iloc { map: None },
            ploc: Ploc { map: None }
        };

        hash_map.iloc.map = unsafe { 
            Some(std::ptr::NonNull::new_unchecked(&mut hash_map.map).as_ref()) 
        };
        hash_map.ploc.map = unsafe { 
            Some(std::ptr::NonNull::new_unchecked(&mut hash_map.map).as_ref()) 
        };
        hash_map
    }
}

#[cfg(test)]
mod tests {
    use super::ExtendedHashMap;
    use std::collections::HashMap;

    const TEST_DATA_LEN: usize = 9;
    const TEST_DATA: [(&str, i32); TEST_DATA_LEN] = [
        ("value1", 1),
        ("value2", 2),
        ("value3", 3),
        ("1", 10),
        ("2", 10),
        ("3", 10),
        ("1, 5", 100),
        ("5, 5", 200),
        ("10, 5", 300)
    ];

    #[test]
    fn create() {
        assert_eq!(
            ExtendedHashMap::<&str, i32>::new().map, 
            HashMap::<&str, i32>::new()
        );

        assert_eq!(
            ExtendedHashMap::<&str, i32>::default().map, 
            HashMap::<&str, i32>::default()
        );
    }

    #[test]
    fn create_from() {
        assert_eq!(
            ExtendedHashMap::from(TEST_DATA).map,
            HashMap::from(TEST_DATA)
        );
    }

    #[test]
    fn len() {
        assert_eq!(
            ExtendedHashMap::from(TEST_DATA).len(),
            TEST_DATA_LEN
        );
    }

    #[test]
    fn empty() {
        let mut map = ExtendedHashMap::<&str, i32>::new();

        assert_eq!(map.is_empty(), true);

        map.insert("1", 1);

        assert_ne!(map.is_empty(), true);
    }

    #[test]
    fn capacity() {
        assert_eq!(
            ExtendedHashMap::<&str, i32>::new().capacity(),
            0
        );
    }

    #[test]
    fn clear() {
        let mut map = ExtendedHashMap::from(TEST_DATA);
        map.clear();

        assert_eq!(map.len(), 0);
        assert_ne!(map.capacity(), 0);
    }

    #[test]
    fn iloc() {
        let map = ExtendedHashMap::from(TEST_DATA);

        let mut sorted_test_data = TEST_DATA.clone();
        sorted_test_data.sort_by(|a, b| a.0.cmp(b.0));

        for i in 0..TEST_DATA_LEN {
            assert_eq!(map.iloc[i], sorted_test_data[i].1);
        }
    }
}