use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::trigger_relic_counter;
use crate::types::RelicName;
use crate::utils::has_relic;

// Queued ahead of every reshuffle: The Abacus's block and Sundial's count, as the reshuffle is set up
pub fn process_effect_shuffle_relic_hooks(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_shuffle_relic_hooks outside the Combat frame"
    );

    // Abacus: reshuffling the discard pile grants 6 block
    if has_relic(&state.id_relics, RelicName::Abacus) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Persistent reshuffle counter
    if let Some(id) = state.id_relics[RelicName::Sundial as usize]
        && trigger_relic_counter(&mut state.entities[id])
    {
        for &effect in state.entities[id].relic_effects_counter {
            state.effect_queue.push_back(effect);
        }
    }
}
