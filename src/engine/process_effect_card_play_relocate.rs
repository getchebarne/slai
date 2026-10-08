use rand::Rng;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardPile;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_card_play_relocate(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardPlayRelocate requires id_target");

    let effect_kind = if state.entities[id_card].card_kind == CardKind::Power {
        // Powers are removed from play
        EffectKind::CardRemove
    } else if state.entities[id_card].card_exhaust
        // Strange Spoon: exhausts have a 50% chance of being discarded instead
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
        kind: effect_kind,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
