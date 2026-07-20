use rand::Rng;

use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_scrap_ooze_reach(
    state: &mut GameState,
    dmg: u16,
    chance: u8,
    advance_on_miss: bool,
) {
    let id_character = state.id_character;
    state.effect_buf.clear();
    state.effect_buf.push(Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(dmg),
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });

    // Success is (chance+1)-in-100; the 105 rung cannot fail
    let roll = state.rng.random_range(0..100) as u8;
    if roll as u16 + chance as u16 >= 99 {
        state.effect_buf.push(Effect {
            kind: EffectKind::RelicGrantRandom,
            id_source: None,
            target: Target::Direct(None),
        });
        state.effect_buf.push(Effect {
            kind: EffectKind::EventConsume,
            id_source: None,
            target: Target::Direct(None),
        });
    } else if advance_on_miss {
        state.effect_buf.push(Effect {
            kind: EffectKind::EventAdvanceState { delta: 1 },
            id_source: None,
            target: Target::Direct(None),
        });
    }
    flush_effects_from_buf_to_queue_front(state);
}
