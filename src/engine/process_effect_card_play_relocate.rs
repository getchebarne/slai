use rand::Rng;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardPile;
use crate::types::RelicName;
use crate::utils::has_relic;

// UseCardAction's pile routing once a Card has been used: Powers vanish, exhausts may
// Strange-Spoon into a discard, everything else discards
pub fn process_effect_card_play_relocate(
    id_target: Option<usize>,
    state: &mut GameState,
    exhaust: bool,
) {
    let id_card = id_target.expect("CardPlayRelocate requires id_target");
    let kind = if state.entities[id_card].card_kind == CardKind::Power {
        EffectKind::CardRemove
    } else if exhaust
        && !(has_relic(&state.id_relics, RelicName::StrangeSpoon)
            && state.rng.random_range(0..100) < 50)
    {
        EffectKind::CardExhaust
    } else {
        // Not a real discard: skips this_turn_discards and on discard triggers
        EffectKind::CardMove {
            pile: CardPile::Discard,
            cost_zero: None,
        }
    };
    state.effect_queue.push_front(Effect {
        kind,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
