use std::hash::{Hash, Hasher};

use fnv::FnvHasher;

pub fn hash_str(name: &str) -> u64 {
    let mut hasher = FnvHasher::default();
    name.hash(&mut hasher);
    hasher.finish()
}
