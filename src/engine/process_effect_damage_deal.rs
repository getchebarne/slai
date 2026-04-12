use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::state::Vitals;
use crate::types::EntityId;

pub fn process_effect_damage_deal(
    vitals: &mut Vitals,
    target: EntityId,
    amount: u16,
) -> ProcessEffectResult {
    // Absorb w/ block, pass remainder as health loss
    let damage_over_block = amount.saturating_sub(vitals.block);
    vitals.block = vitals.block.saturating_sub(amount);

    if damage_over_block > 0 {
        ProcessEffectResult::AddAndContinue {
            top: vec![Effect {
                kind: EffectKind::HealthLoss {
                    amount: damage_over_block,
                },
                source: None,
                target: Target::Direct(Some(target)),
            }],
            bot: Vec::new(),
        }
    } else {
        ProcessEffectResult::Continue
    }
}
