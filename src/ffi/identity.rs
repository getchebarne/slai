use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

// Hash of a snapshot object whose `id` and `identity_hash` still hold 0, so it leaves the id out
pub(crate) fn identity_hash<T: Hash>(snapshot: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    snapshot.hash(&mut hasher);
    hasher.finish()
}
