use std::{
    collections::HashMap,
    hash::Hash,
    ops::Index
};

#[derive(Debug)]
pub struct Iloc<'a, K: 'a, V: 'a> {
    pub(crate) map: Option<&'a HashMap<K, V>>
}

impl<'a, K, V> Index<usize> for Iloc<'a, K, V>
where
    K: Ord + Hash
{
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(map) = self.map {
            let mut keys = map.keys().collect::<Vec<_>>();
            keys.sort();

            if let Some(key) = keys.iter().nth(index) {
                return map.get(key).unwrap();
            }
        }
        panic!("No key with index {} was found", index);
    }
}