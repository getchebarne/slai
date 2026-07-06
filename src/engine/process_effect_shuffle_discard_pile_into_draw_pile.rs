use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::relic_counter_fire;
use crate::types::RelicName;
use crate::utils::reshuffle_discard_into_draw;

pub fn process_effect_shuffle_discard_pile_into_draw_pile(state: &mut GameState) {
    reshuffle_discard_into_draw(
        &mut state.id_pile_draw,
        &mut state.id_pile_discard,
        &mut state.rng,
    );

    // Relic-sourced block: id_source None skips Dex/Frail scaling
    if state.id_relics[RelicName::Abacus as usize].is_some() {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }
    // Persistent reshuffle counter; every 3rd fires
    if relic_counter_fire(RelicName::Sundial, 3, &state.id_relics, &mut state.entities) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::EnergyGain { amount: 2 },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
