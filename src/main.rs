use subimpl::HashMapDelegate;
use std::collections::HashMap;

#[derive(HashMapDelegate)]
struct Map {
    map: HashMap<String, String>,
}

pub fn main() {
    let mut map = Map::new();
    map.insert("key".to_string(), "value".to_string());
    println!("{:?}", map.get(&"key".to_string()));
}