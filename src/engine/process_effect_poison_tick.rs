use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::types::DeltaSign;

// HP loss = Poison stacks; then Poison -= 1 (remove on 0). Fires at turn start
pub fn process_effect_poison_tick(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("PoisonTick requires id_target");
    let modifiers = &mut state.entities[id_target].modifiers;
    if !modifier_has(modifiers, ModifierKind::Poison) {
        return;
    }
    let stacks = modifier_stacks(modifiers, ModifierKind::Poison);

    if stacks <= 1 {
        modifier_remove(modifiers, ModifierKind::Poison);
    } else {
        modifiers.stacks[ModifierKind::Poison as usize] = stacks - 1;
    }

    state.effect_queue.push_front(Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(stacks as u16),
        },
        id_source: None,
        target: Target::Direct(Some(id_target)),
    });
}
