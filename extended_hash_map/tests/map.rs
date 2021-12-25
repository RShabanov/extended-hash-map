use extended_hash_map::ExtendedHashMap;
use std::collections::{hash_map, HashMap};

const TEST_DATA_LEN: usize = 12;
const TEST_DATA: [(&str, i32); TEST_DATA_LEN] = [
    ("value1", 1),
    ("value2", 2),
    ("value3", 3),
    ("1", 10),
    ("2", 10),
    ("3", 10),
    ("1, 5", 100),
    ("5, 5", 200),
    ("10, 5", 300),
    ("(1, 5, 3)", 400),
    ("(5, 5, 4)", 500),
    ("(10, 5, 5)", 600),
];

fn ploc_conditions_for_test_data() -> (Vec<&'static str>, Vec<HashMap<&'static str, i32>>) {
    (
        vec![">=1", "<3", ">0, >0", ">=10, >0", "<5, >=5, >=3"],
        vec![
            HashMap::from([("1", 10), ("2", 10), ("3", 10)]),
            HashMap::from([("1", 10), ("2", 10)]),
            HashMap::from([("1, 5", 100), ("5, 5", 200), ("10, 5", 300)]),
            HashMap::from([("10, 5", 300)]),
            HashMap::from([("(1, 5, 3)", 400)]),
        ],
    )
}

fn hash_map_iter() -> std::vec::IntoIter<(&'static str, i32)> {
    let mut hash_map = HashMap::from(TEST_DATA).into_iter().collect::<Vec<_>>();

    hash_map.sort_by(|lhs, rhs| lhs.0.cmp(rhs.0));

    hash_map.into_iter()
}

#[test]
fn create() {
    assert_eq!(ExtendedHashMap::<&str, i32>::new().is_empty(), true);

    assert_eq!(ExtendedHashMap::<&str, i32>::default().is_empty(), true);
}

#[test]
fn create_from() {
    let map = ExtendedHashMap::from(TEST_DATA);
    let hash_map = HashMap::from(TEST_DATA);

    for (key, val) in map.iter() {
        assert_eq!(hash_map[key], *val);
    }
}

#[test]
fn len() {
    assert_eq!(ExtendedHashMap::from(TEST_DATA).len(), TEST_DATA_LEN);
}

#[test]
fn is_empty() {
    let mut map = ExtendedHashMap::<&str, i32>::new();

    assert_eq!(map.is_empty(), true);

    map.insert("1", 1);

    assert_eq!(map.is_empty(), false);
}

#[test]
fn capacity() {
    assert_eq!(ExtendedHashMap::<&str, i32>::new().capacity(), 0);
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

#[test]
fn insert() {
    let mut map = ExtendedHashMap::new();
    map.insert("1", 1);

    assert_eq!(map.len(), 1);
    assert_eq!(map["1"], 1);
}

#[test]
fn ploc() {
    let map = ExtendedHashMap::from(TEST_DATA);
    let (conditions, hash_maps) = ploc_conditions_for_test_data();

    for (condition, hash_map) in conditions.iter().zip(hash_maps.iter()) {
        assert_eq!(map.ploc(condition), *hash_map);
    }
}

#[test]
fn iter() {
    let map = ExtendedHashMap::from(TEST_DATA);

    for ((lhs_key, lhs_val), (rhs_key, rhs_val)) in map.iter().zip(hash_map_iter()) {
        assert_eq!(*lhs_key, rhs_key);
        assert_eq!(*lhs_val, rhs_val);
    }
}

#[test]
fn keys() {
    let map = ExtendedHashMap::from(TEST_DATA);

    for (lhs_key, (rhs_key, _)) in map.keys().zip(hash_map_iter()) {
        assert_eq!(*lhs_key, rhs_key);
    }
}

#[test]
fn values() {
    let map = ExtendedHashMap::from(TEST_DATA);

    for (lhs_val, (_, rhs_val)) in map.values().zip(hash_map_iter()) {
        assert_eq!(*lhs_val, rhs_val);
    }
}
