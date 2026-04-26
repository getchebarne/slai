use crate::effect::EffectKind;
use crate::engine::DispatchResult;
use crate::entity::Entity;

// Glass Knife (and any future self-decaying card): mutate the played card's
// own DamagePhysical amounts in-place by `delta`. Reads `last_played_card`
// to find which entity to mutate. Saturates at 0 so the card eventually
// deals no damage but can still be played.
pub fn process_effect_glass_knife_decay(
    entities: &mut [Entity],
    last_played_card: Option<usize>,
    delta: i16,
) -> DispatchResult {
    let id_card = match last_played_card {
        Some(id) => id,
        None => return DispatchResult::Continue,
    };
    let card = &mut entities[id_card];
    let len = card.card_effects_len as usize;
    for effect in card.card_effects[..len].iter_mut() {
        if let EffectKind::DamagePhysical { amount } = &mut effect.kind {
            let new = (*amount as i32 + delta as i32).max(0).min(u16::MAX as i32);
            *amount = new as u16;
        }
    }
    DispatchResult::Continue
}
