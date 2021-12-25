# extended-hash-map lib

## Examples

<hr>

### ExtendedHashMap::iloc:
```rust
use extended_hash_map::ExtendedHashMap;

fn main() {
    let mut map = ExtendedHashMap::from([
        ("value1", 1),
        ("value2", 2),
        ("value3", 3),
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("1, 5", 100),
        ("5, 5", 200),
        ("10, 5", 300),
    ]);

    println!("{}", map.iloc[0]);
    println!("{}", map.iloc[2]);
    println!("{}", map.iloc[5]);
    println!("{}", map.iloc[8]);
}
```
Output:
```
10
300
200
3
```

<hr>

### ExtendedHashMap::ploc:
```rust
use extended_hash_map::ExtendedHashMap;

fn main() {
    let mut map = ExtendedHashMap::from([
        ("value1", 1),
        ("value2", 2),
        ("value3", 3),
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("1, 5", 100),
        ("5, 5", 200),
        ("10, 5", 300),
    ]);

    println!("{:?}", map.ploc(">=1"));
    println!("{:?}", map.ploc("<3"));
    println!("{:?}", map.ploc(">0, >0"));
    println!("{:?}", map.ploc(">=10, >0"));
    println!("{:?}", map.ploc("<5, >=5, >=3"));
}
```
Output:
```
{"1": 10, "3": 10, "2": 10}
{"2": 10, "1": 10}
{"5, 5": 200, "1, 5": 100, "10, 5": 300}
{"10, 5": 300}
{"(1, 5, 3)": 400}
```
