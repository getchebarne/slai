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

#[cfg(test)]
mod tests {
    use crate::effect::Effect;
    use crate::effect::EffectKind;
    use crate::effect::Target;
    use crate::engine::process_effect_queue;
    use crate::engine::test_support::combat_with_relic;
    use crate::engine::test_support::set_relic_counter;
    use crate::game::GameState;
    use crate::types::MonsterName;
    use crate::types::RelicName;

    // Empty the draw pile into discard, then draw to force a reshuffle
    fn force_reshuffle(state: &mut GameState) {
        let drained: Vec<usize> = state.id_pile_draw.drain(..).collect();
        state.id_pile_discard.extend(drained);
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        });
        process_effect_queue(state);
    }

    #[test]
    fn abacus_blocks_on_reshuffle() {
        let mut state = combat_with_relic(RelicName::Abacus, MonsterName::JawWorm);
        force_reshuffle(&mut state);
        assert_eq!(state.entities[state.id_character].vitals.block, 6);
    }

    #[test]
    fn sundial_grants_energy_every_third_reshuffle() {
        let mut state = combat_with_relic(RelicName::Sundial, MonsterName::JawWorm);
        set_relic_counter(&mut state, RelicName::Sundial, 2);
        force_reshuffle(&mut state);
        assert_eq!(state.energy.energy_current, 5);
        let id = state.id_relics[RelicName::Sundial as usize].unwrap();
        assert_eq!(state.entities[id].relic_counter, 0);
    }
}
