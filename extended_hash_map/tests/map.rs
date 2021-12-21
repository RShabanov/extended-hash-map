use extended_hash_map::ExtendedHashMap;

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