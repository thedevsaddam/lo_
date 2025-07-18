use std::collections::HashMap;
use std::hash::Hash;

/// Returns a new map with only the specified keys retained.
///
/// # Example
/// ```rust
/// use lo_::pick_keys;
/// use std::collections::HashMap;
/// let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
/// let result = pick_keys(&map, ["a", "b"]);
/// assert_eq!(result, HashMap::from([("a", 1), ("b", 2)]));
/// ```
pub fn pick_keys<K, V, I>(map: &HashMap<K, V>, keys: I) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
    I: IntoIterator<Item = K>,
{
    let pick_set: std::collections::HashSet<_> = keys.into_iter().collect();
    map.iter()
        .filter(|(k, _)| pick_set.contains(*k))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_keys() {
        let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
        let result = pick_keys(&map, vec!["a", "c"]);
        let expected = HashMap::from([("a", 1), ("c", 3)]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pick_keys_nonexistent() {
        let map = HashMap::from([("x", 42)]);
        let result = pick_keys(&map, vec!["a", "x"]);
        assert_eq!(result, HashMap::from([("x", 42)]));
    }
}
