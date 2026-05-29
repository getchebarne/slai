use rand::Rng;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::types::DeltaSign;
use crate::effect::Target;
use crate::game::GameState;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_scrap_ooze_reach(
    id_source: Option<usize>,
    state: &mut GameState,
    dmg: u16,
    chance: u8,
    advance_on_miss: bool,
) {
    let id_character = state.id_character;
    state.buf_effects.clear();
    state.buf_effects.push(Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: HealthDeltaAmount::Absolute(dmg),
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });

    let roll = state.rng.random_range(0..100) as u8;
    if roll < chance {
        state.buf_effects.push(Effect {
            kind: EffectKind::RelicGrantRandom,
            id_source: None,
            target: Target::Direct(None),
        });
        state.buf_effects.push(Effect {
            kind: EffectKind::EventEnd,
            id_source,
            target: Target::Direct(None),
        });
    } else if advance_on_miss {
        state.buf_effects.push(Effect {
            kind: EffectKind::EventAdvanceState { delta: 1 },
            id_source,
            target: Target::Direct(None),
        });
    }
    flush_effects_from_buf_to_queue_front(state);
}
