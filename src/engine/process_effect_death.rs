use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::monsters::Monster;
use crate::types::ActorId;

pub fn process_effect_death(
    actor: ActorId,
    monsters: &mut Vec<Monster>,
) -> ProcessEffectResult {
    match actor {
        ActorId::Character => ProcessEffectResult::Continue {
            top: vec![Effect::GameEnd],
            bot: Vec::new(),
        },
        ActorId::Monster(i) => {
            let idx = i as usize;
            let mut effects = Vec::new();

            if modifier_has(&monsters[idx].vitals.modifiers, ModifierKind::SporeCloud) {
                let stacks =
                    modifier_stacks(&monsters[idx].vitals.modifiers, ModifierKind::SporeCloud);
                effects.push(Effect::ModifierGain {
                    target: ActorId::Character,
                    kind: ModifierKind::Vulnerable,
                    stacks,
                });
            }
 
            monsters.remove(idx);

            if monsters.is_empty() {
                effects.push(Effect::CombatEnd);
            }

            if effects.is_empty() {
                ProcessEffectResult::Pass
            } else {
                ProcessEffectResult::Continue {
                    top: effects,
                    bot: Vec::new(),
                }
            }
        }
    }
}
