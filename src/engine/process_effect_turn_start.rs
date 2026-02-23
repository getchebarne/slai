use crate::consts::CARDS_DRAWN_PER_TURN;
use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, modifier_has, modifier_remove, modifier_stacks};
use crate::state::{Energy, Vitals};
use crate::types::ActorId;

pub fn process_effect_turn_start(
    vitals: &mut Vitals,
    actor: ActorId,
    energy: &Energy,
    num_monsters: usize,
) -> ProcessEffectResult {
    let mut effects = Vec::new();

    let mut new_block: u16 = 0;
    if modifier_has(&vitals.modifiers, ModifierKind::Blur) {
        new_block += vitals.block;
    }
    if modifier_has(&vitals.modifiers, ModifierKind::NextTurnBlock) {
        new_block += modifier_stacks(&vitals.modifiers, ModifierKind::NextTurnBlock) as u16;
        modifier_remove(&mut vitals.modifiers, ModifierKind::NextTurnBlock);
    }
    effects.push(Effect::BlockSet { target: actor, amount: new_block });

    if modifier_has(&vitals.modifiers, ModifierKind::Phantasmal) {
        effects.push(Effect::ModifierGain {
            target: actor,
            kind: ModifierKind::DoubleDamage,
            stacks: 1,
        });
    }

    if actor == ActorId::Character {
        effects.push(Effect::CardDraw { count: CARDS_DRAWN_PER_TURN });
        let energy_gain = energy.max - energy.current;
        effects.push(Effect::EnergyGain { amount: energy_gain });
        effects.push(Effect::ModifierTick { target: ActorId::Character });
        for i in 0..num_monsters {
            effects.push(Effect::ModifierTick { target: ActorId::Monster(i as u8) });
        }

        if modifier_has(&vitals.modifiers, ModifierKind::NextTurnEnergy) {
            let stacks = modifier_stacks(&vitals.modifiers, ModifierKind::NextTurnEnergy);
            effects.push(Effect::EnergyGain { amount: stacks as u8 });
            modifier_remove(&mut vitals.modifiers, ModifierKind::NextTurnEnergy);
        }

        if modifier_has(&vitals.modifiers, ModifierKind::InfiniteBlades) {
            let stacks = modifier_stacks(&vitals.modifiers, ModifierKind::InfiniteBlades);
            effects.push(Effect::AddShivs { count: stacks as u8 });
        }
    }

    ProcessEffectResult::Continue {
        top: effects,
        bot: Vec::new(),
    }
}
