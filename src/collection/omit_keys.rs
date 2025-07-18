use std::collections::HashMap;
use std::hash::Hash;

/// Returns a new map with specified keys removed.
///
/// # Example
/// ```rust
/// use lo_::omit_keys;
/// use std::collections::HashMap;
/// let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
/// let result = omit_keys(&map, ["a", "c"]);
/// assert_eq!(result, HashMap::from([("b", 2)]));
/// ```
pub fn omit_keys<K, V, I>(map: &HashMap<K, V>, keys: I) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
    I: IntoIterator<Item = K>,
{
    let omit_set: std::collections::HashSet<_> = keys.into_iter().collect();
    map.iter()
        .filter(|(k, _)| !omit_set.contains(*k))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omit_keys() {
        let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
        let result = omit_keys(&map, vec!["a", "c"]);
        let expected = HashMap::from([("b", 2)]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_omit_keys_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        let result = omit_keys(&map, vec!["a"]);
        assert!(result.is_empty());
    }
}
