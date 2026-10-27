use crate::effect::EFFECT_ACCURACY_RESYNC_HAND;
use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::CardPile;
use crate::utils::place_card;

// A Card made for this combat (new, copied, or back from Stasis) enters a pile; a full hand overflows to discard
pub fn process_effect_card_place(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    pile: CardPile,
) {
    let id_target = first_target(state, EffectKind::CardPlace { pile }, id_source, id_targets);
    let id_card = id_target.expect("CardPlace requires id_target");
    let reached_hand = place_card(state, id_card, pile) && pile == CardPile::Hand;

    // Accuracy: a Card reaching the hand resets every Shiv in it
    if reached_hand
        && has_modifier(
            &state.entities[state.id_character].modifiers,
            ModifierKind::Accuracy,
        )
    {
        state.effect_queue.push_front(EFFECT_ACCURACY_RESYNC_HAND);
    }
}
