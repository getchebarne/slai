use crate::effect::Effect;
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
        ProcessEffectResult::Continue {
            top: vec![Effect::HealthLoss {
                target,
                amount: damage_over_block,
            }],
            bot: Vec::new(),
        }
    } else {
        ProcessEffectResult::Pass
    }
}
