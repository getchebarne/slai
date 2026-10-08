use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::types::DeltaSign;

// HP loss = the stacks held when the tick was queued; then Poison -= 1 (removed at 0)
pub fn process_effect_poison_tick(id_target: Option<usize>, state: &mut GameState, amount: u16) {
    let id_target = id_target.expect("PoisonTick requires id_target");

    // Executes in reverse:
    //     1. HealthDelta (a kill still holds every stack for The Specimen)
    //     2. ModifierGain Poison -1
    state.effect_queue.push_front(Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: -1,
        },
        id_source: None,
        target: Target::Direct(Some(id_target)),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: Target::Direct(Some(id_target)),
    });
}
