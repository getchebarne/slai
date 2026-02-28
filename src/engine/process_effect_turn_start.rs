use crate::consts::CARDS_DRAWN_PER_TURN;
use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_remove, modifier_stacks};
use crate::state::{Energy, Vitals};
use crate::types::EntityId;

pub fn process_effect_turn_start(
    vitals: &mut Vitals,
    modifiers: &mut Modifiers,
    actor: EntityId,
    energy: &Energy,
    monster_ids: &[EntityId],
) -> ProcessEffectResult {
    let mut effects = Vec::new();

    let mut new_block: u16 = 0;
    if modifier_has(modifiers, ModifierKind::Blur) {
        new_block += vitals.block;
    }
    if modifier_has(modifiers, ModifierKind::NextTurnBlock) {
        new_block += modifier_stacks(modifiers, ModifierKind::NextTurnBlock) as u16;
        modifier_remove(modifiers, ModifierKind::NextTurnBlock);
    }
    effects.push(Effect::BlockSet { target: actor, amount: new_block });

    if modifier_has(modifiers, ModifierKind::Phantasmal) {
        effects.push(Effect::ModifierGain {
            target: actor,
            kind: ModifierKind::DoubleDamage,
            stacks: 1,
        });
    }

    if actor.0 == 0 {
        effects.push(Effect::CardDraw { count: CARDS_DRAWN_PER_TURN });
        let energy_gain = energy.max - energy.current;
        effects.push(Effect::EnergyGain { amount: energy_gain });
        effects.push(Effect::ModifierTick { target: EntityId(0) });
        for &mid in monster_ids {
            effects.push(Effect::ModifierTick { target: mid });
        }

        if modifier_has(modifiers, ModifierKind::NextTurnEnergy) {
            let stacks = modifier_stacks(modifiers, ModifierKind::NextTurnEnergy);
            effects.push(Effect::EnergyGain { amount: stacks as u8 });
            modifier_remove(modifiers, ModifierKind::NextTurnEnergy);
        }

        if modifier_has(modifiers, ModifierKind::InfiniteBlades) {
            let stacks = modifier_stacks(modifiers, ModifierKind::InfiniteBlades);
            effects.push(Effect::AddShivs { count: stacks as u8 });
        }
    }

    ProcessEffectResult::Continue {
        top: effects,
        bot: Vec::new(),
    }
}
