use crate::engine::DispatchResult;
use crate::entity::Entity;

// Silently remove a monster from combat: flag it dead WITHOUT firing the
// on-death hook chain (no SporeCloud, no CorpseExplosion, no boss-victory).
// Used by the Large Slimes' Split move to remove the L after spawning its
// mediums; reused at Tier 5 by the Looter for its actual flee.
pub fn process_effect_escape_monster(id_target: usize, entities: &mut [Entity]) -> DispatchResult {
    entities[id_target].dead = true;
    DispatchResult::Continue
}
