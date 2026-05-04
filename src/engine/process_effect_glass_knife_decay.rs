use crate::effect::EffectKind;
use crate::engine::DispatchResult;
use crate::entity::Entity;

// Glass Knife mutate the played card's own DamagePhysical amounts in-place
// by `delta`. Reads `card_last_played`
pub fn process_effect_glass_knife_decay(
    entities: &mut [Entity],
    card_last_played: Option<usize>,
    delta: i16,
) -> DispatchResult {
    let id_card = card_last_played.expect("GlassKnifeDecay with no card_last_played");
    let card = &mut entities[id_card];
    let num_effects = card.card_effects_len as usize;
    for effect in card.card_effects[..num_effects].iter_mut() {
        if let EffectKind::DamagePhysical { amount, .. } = &mut effect.kind {
            let new = (*amount as i32 + delta as i32).max(0).min(u16::MAX as i32);
            *amount = new as u16;
        }
    }
    DispatchResult::Continue
}
