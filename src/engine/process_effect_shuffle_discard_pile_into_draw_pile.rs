use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::trigger_relic_counter;
use crate::types::Combat;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::shuffle;

pub fn process_effect_shuffle_discard_pile_into_draw_pile(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_shuffle_discard_pile_into_draw_pile outside the Combat frame"
    );
    let Combat {
        id_card_draw,
        id_card_discard,
        ..
    } = &mut state.combat;
    id_card_draw.append(id_card_discard);
    shuffle(&mut id_card_draw[..], &mut state.rng);

    // Abacus: reshuffling the discard pile grants 6 block
    if has_relic(&state.id_relics, RelicName::Abacus) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Persistent reshuffle counter
    if let Some(id) =
        trigger_relic_counter(RelicName::Sundial, &state.id_relics, &mut state.entities)
    {
        for &effect in state.entities[id].relic_effects_counter {
            state.effect_queue.push_back(effect);
        }
    }
}
