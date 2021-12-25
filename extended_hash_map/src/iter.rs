use std::{collections::hash_map as base, vec};

#[derive(Debug, Clone)]
pub struct Iter<'a, K: 'a, V: 'a> {
    pub(crate) base: vec::IntoIter<(&'a K, &'a V)>,
}

#[derive(Debug, Clone)]
pub struct Keys<'a, K: 'a, V: 'a> {
    pub(crate) inner: Iter<'a, K, V>,
}

#[derive(Debug, Clone)]
pub struct Values<'a, K: 'a, V: 'a> {
    pub(crate) inner: Iter<'a, K, V>,
}

impl<'a, K, V> From<base::Iter<'a, K, V>> for Iter<'a, K, V>
where
    K: Ord,
{
    fn from(iter: base::Iter<'a, K, V>) -> Self {
        let mut base: Vec<(&'a K, &'a V)> = iter.collect();
        base.sort_by(|lhs, rhs| lhs.0.cmp(rhs.0));

        Self {
            base: base.into_iter(),
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.base.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<&'a K> {
        self.inner.next().map(|(k, _)| k)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|(_, v)| v)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
