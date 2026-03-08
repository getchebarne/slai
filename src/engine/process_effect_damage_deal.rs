use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::state::Vitals;
use crate::types::EntityId;

pub fn process_effect_damage_deal(
    vitals: &mut Vitals,
    target: EntityId,
    amount: u16,
) -> ProcessEffectResult {
    let damage_over_block = amount.saturating_sub(vitals.block);
    vitals.block = vitals.block.saturating_sub(amount);

    if damage_over_block > 0 {
        ProcessEffectResult::AddAndContinue {
            top: vec![Effect {
                kind: EffectKind::HealthLoss {
                    amount: damage_over_block,
                },
                source: None,
                target: Some(target),
            }],
            bot: Vec::new(),
        }
    } else {
        ProcessEffectResult::Continue
    }
}
