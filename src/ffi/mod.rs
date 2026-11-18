// FFI package: one file per snapshot family, mirroring the engine layout
mod macros;

pub mod action;
pub mod amount;
pub mod card;
pub mod character;
pub mod context;
pub mod effect;
pub mod event;
pub mod map;
pub mod modifier;
pub mod monster;
pub mod potion;
pub mod relic;
pub mod state;
pub mod target;
pub mod template;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

pub use action::*;
pub use amount::*;
pub use card::*;
pub use character::*;
pub use context::*;
pub use effect::*;
pub use event::*;
pub use map::*;
pub use modifier::*;
pub use monster::*;
pub use potion::*;
pub use relic::*;
pub use state::*;
pub use target::*;
pub use template::*;

// Hash of a snapshot object whose `id` and `identity_hash` still hold 0, so it leaves the id out
pub(crate) fn identity_hash<T: Hash>(snapshot: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    snapshot.hash(&mut hasher);
    hasher.finish()
}
