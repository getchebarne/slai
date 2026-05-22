use crate::effect::EffectKind;
use crate::game::GameState;

// Glass Knife mutates its own DamagePhysical amounts in-place by `delta`.
// `id_target` is the card itself (resolved via CandidatePool::Source from
// the on-play rewrite that sets `id_source = id_card`)
pub fn process_effect_glass_knife_decay(
    id_target: Option<usize>,
    state: &mut GameState,
    delta: i16,
) {
    let id_target = id_target.expect("GlassKnifeDecay requires id_target");
    let card = &mut state.entities[id_target];
    let num_effects = card.card_effects_len as usize;
    for effect in card.card_effects[..num_effects].iter_mut() {
        if let EffectKind::DamagePhysical { amount, .. } = &mut effect.kind {
            let new = (*amount as i32 + delta as i32).max(0).min(u16::MAX as i32);
            *amount = new as u16;
        }
    }
}
