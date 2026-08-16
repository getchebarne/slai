use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;

pub fn process_effect_singing_bowl_proc(state: &mut GameState, idx_bundle: u8) {
    assert!(
        state.reward.active,
        "SingingBowlProc outside the Reward context"
    );

    // Remove bundle
    state.reward.id_cards.remove(idx_bundle as usize);

    // Push effect for max health gain
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(2),
        },
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });
}
