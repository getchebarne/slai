use crate::effect::EffectKind;
use crate::engine::DispatchResult;
use crate::entity::Entity;

// Glass Knife mutates its own DamagePhysical amounts in-place by `delta`.
// `id_source` carries the played card's entity id (set by the card-play
// rewrite at process_effect_card_play.rs)
pub fn process_effect_glass_knife_decay(
    entities: &mut [Entity],
    id_source: Option<usize>,
    delta: i16,
) -> DispatchResult {
    let id_card = id_source.expect("GlassKnifeDecay with no id_source");
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
