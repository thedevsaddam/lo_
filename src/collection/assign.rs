use std::collections::HashMap;
use std::hash::Hash;

/// Merges multiple maps from left to right, where later maps overwrite keys from earlier ones.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use lo_::assign;
///
/// let map1 = HashMap::from([("a", 1), ("b", 2)]);
/// let map2 = HashMap::from([("b", 3), ("c", 4)]);
///
/// let merged = assign(vec![map1, map2]);
///
/// assert_eq!(merged.get("a"), Some(&1)); // preserved
/// assert_eq!(merged.get("b"), Some(&3)); // overwritten
/// assert_eq!(merged.get("c"), Some(&4)); // added
///
/// ```
pub fn assign<K, V>(maps: Vec<HashMap<K, V>>) -> HashMap<K, V>
where
    K: Eq + Hash,
{
    let mut result = HashMap::new();
    for map in maps {
        result.extend(map);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_two_maps() {
        let map1 = HashMap::from([("a", 1), ("b", 2)]);
        let map2 = HashMap::from([("b", 3), ("c", 4)]);

        let result = assign(vec![map1, map2]);

        assert_eq!(result.get("a"), Some(&1));
        assert_eq!(result.get("b"), Some(&3)); // overwritten
        assert_eq!(result.get("c"), Some(&4));
    }

    #[test]
    fn test_assign_multiple_maps() {
        let map1 = HashMap::from([("x", 1)]);
        let map2 = HashMap::from([("x", 2), ("y", 3)]);
        let map3 = HashMap::from([("z", 4)]);

        let result = assign(vec![map1, map2, map3]);

        assert_eq!(result.get("x"), Some(&2));
        assert_eq!(result.get("y"), Some(&3));
        assert_eq!(result.get("z"), Some(&4));
    }

    #[test]
    fn test_assign_empty_input() {
        let result: HashMap<String, i32> = assign(vec![]);
        assert!(result.is_empty());
    }
}
