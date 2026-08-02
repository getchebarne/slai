use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;

// Singing Bowl: forfeit one card bundle for +2 max HP (no heal)
pub fn process_effect_singing_bowl_proc(state: &mut GameState, idx_bundle: u8) {
    let Some(Mode::Reward {
        reward_id_cards: bundles,
        ..
    }) = state.mode_stack.last_mut()
    else {
        unreachable!("SingingBowlProc outside Reward mode")
    };
    bundles.remove(idx_bundle as usize);
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(2),
        },
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });
}
