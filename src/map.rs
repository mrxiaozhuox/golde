use std::{collections::HashMap, fmt::{Display, Debug}, hash::Hash};

#[derive(Clone, Debug)]
pub struct Map<K, V> {
    pub(crate) inner: HashMap<K, V>
}

impl<K: Eq + Hash, V> Map<K, V> {

    pub fn new() -> Map<K, V> {
        return Map {
            inner: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: K, v: V) -> Option<V> {
        self.inner.insert(k, v)
    }

    pub fn get(&self, k: K) -> Option<&V> {
        return self.inner.get(&k);
    }

    pub fn len(&self) -> usize { self.inner.len() }

}

impl<K, V> Display for Map<K, V> 
where
    K: Debug + std::cmp::Eq + std::hash::Hash + serde::ser::Serialize,
    V: Debug + serde::ser::Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(
            &self.inner
        ).unwrap_or("{}".to_string()))
    }
}

#[test]
fn test_map_display() {
    let map: Map<&str, doson::DataValue> = Map::new();
    println!("{}", map);
}