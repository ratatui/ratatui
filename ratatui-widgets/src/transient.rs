use core::hash::{Hash, Hasher};

/// A wrapper for transient (runtime-only) state excluded from equality and hashing.
///
/// Two `Transient` values are always considered equal, and hashing is a no-op.
/// This lets structs with cached/derived fields derive `PartialEq`, `Eq`, and `Hash`.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Transient<T>(pub T);

impl<T> PartialEq for Transient<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for Transient<T> {}

impl<T> Hash for Transient<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

#[cfg(test)]
mod tests {
    use core::hash::{Hash, Hasher};

    use super::Transient;

    #[test]
    fn always_equal_regardless_of_inner_value() {
        assert_eq!(Transient(1), Transient(2));
        assert_eq!(Transient(Some(10)), Transient(None));
    }

    #[test]
    fn hash_is_noop() {
        let hash = |t: &Transient<i32>| {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            t.hash(&mut hasher);
            hasher.finish()
        };

        assert_eq!(hash(&Transient(1)), hash(&Transient(2)));
        // Also verify it matches the hash of nothing
        let empty_hash = {
            let hasher = std::collections::hash_map::DefaultHasher::new();
            hasher.finish()
        };
        assert_eq!(hash(&Transient(42)), empty_hash);
    }
}
