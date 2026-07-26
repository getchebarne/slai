use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;

// Queued right after the card's damage: the kill has fully resolved by the time this runs
pub fn process_effect_hand_of_greed_proc(
    id_target: Option<usize>,
    state: &mut GameState,
    gold: u16,
) {
    let id_target = id_target.expect("HandOfGreedProc requires id_target");
    if state.entities[id_target].dead {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(gold),
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
