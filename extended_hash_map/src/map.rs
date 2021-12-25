use std::{
    collections::HashMap,
    hash::Hash,
    ops::Index,
    borrow::Borrow, fmt::Debug
};
use super::{
    iloc::Iloc,
    expr_ast::{
        Parser,
        node::{
            Tree,
            BinOp,
            Node
        },
        token::{
            literal::Literal,
            op::OpKind
        }
    },
    iter::*
};

#[derive(Debug)]
pub struct ExtendedHashMap<'a, K, V> {
    map: HashMap<K, V>,
    pub iloc: Iloc<'a, K, V>,
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

    fn tree_state(&self, tree: &Tree) -> bool {
        for node in tree.root.iter() {
            match node {
                Node::BinOp(bin_op) => {
                    let BinOp{lhs, op, rhs} = bin_op; 
                    let lhs: f64 = 
                        match lhs {
                            Literal::Integer(s) | Literal::Float(s) => s.parse().unwrap()
                        };

                    let rhs: f64 = 
                        match rhs {
                            Literal::Integer(s) | Literal::Float(s) => s.parse().unwrap()
                        };
                    
                    return match op {
                        OpKind::Eq => lhs == rhs,
                        OpKind::Ne => lhs != rhs,
                        OpKind::Ge => lhs >= rhs,
                        OpKind::Gt => lhs > rhs,
                        OpKind::Le => lhs <= rhs,
                        OpKind::Lt => lhs < rhs,
                    };
                },
                _ => return false
            }
        }
        false
    }

    fn mix_trees(&self, lhs: &mut Tree, rhs: Tree) {
        let mut rhs = rhs;
        for i in 0..lhs.len() {
            match lhs.root[i] {
                Node::BinOp(ref mut bin_op) => {
                    match rhs.root[i] {
                        Node::Literal(ref mut lit) => {
                            std::mem::swap(&mut bin_op.lhs, lit);
                        },
                        _ => panic!()
                    }
                },
                _ => panic!()
            }
        }
    }
}

impl<K, V> ExtendedHashMap<'_, K, V>
where
    K: Ord
{
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::from(self.map.iter())
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values { inner: self.iter() }
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys { inner: self.iter() }
    }
}

impl<V> ExtendedHashMap<'_, &str, V>
where
    V: Clone + Debug
{
    pub fn ploc(&self, statement: &str) -> HashMap<&str, V> {
        let mut parser = Parser::new();
        let mut new_hash_map = HashMap::new();

        let mut tree = 
            match parser.parse_condition(statement) {
                Ok(tree) => tree,
                Err(_) => return new_hash_map
            };

        for (key, val) in self.iter() {
            if key
                .chars()
                .any(|ch| ch.is_alphabetic()) 
            {
                continue;
            }

            let key_tree = 
                match parser.parse(key) {
                    Ok(key_tree) => key_tree,
                    Err(_) => continue
                };

            if tree.len() == key_tree.len() {
                self.mix_trees(&mut tree, key_tree);

                if self.tree_state(&tree) {
                    new_hash_map.insert(key.clone(), val.clone());
                }
            }
        }
        
        new_hash_map
    }
}

impl<K, V> Default for ExtendedHashMap<'_, K, V>
{
    #[inline]
    fn default() -> Self {
        let mut hash_map = Self {
            map: HashMap::<K, V>::default(),
            iloc: Iloc { map: None },
        };

        hash_map.iloc.map = unsafe { 
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
        };

        hash_map.iloc.map = unsafe { 
            Some(std::ptr::NonNull::new_unchecked(&mut hash_map.map).as_ref()) 
        };

        hash_map
    }
}
