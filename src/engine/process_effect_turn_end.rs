use crate::effect::Effect;
use crate::engine::{ProcessEffectResult, instantiate_templates};
use crate::modifier::{ModifierKind, modifier_has, modifier_remove, modifier_stacks};
use crate::monsters::Monster;
use crate::state::Vitals;
use crate::types::ActorId;

pub fn process_effect_turn_end_monster(
    vitals: &mut Vitals,
    actor: ActorId,
) -> ProcessEffectResult {
    if modifier_has(&vitals.modifiers, ModifierKind::Ritual)
        && !vitals.modifiers.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(&vitals.modifiers, ModifierKind::Ritual);
        return ProcessEffectResult::Continue {
            top: vec![Effect::ModifierGain {
                target: actor,
                kind: ModifierKind::Strength,
                stacks,
            }],
            bot: Vec::new(),
        };
    }
    ProcessEffectResult::Pass
}

pub fn process_effect_turn_end_character(
    character_vitals: &mut Vitals,
    monsters: &[Monster],
    card_target: Option<u8>,
) -> ProcessEffectResult {
    let mut effects = Vec::new();

    if modifier_has(&character_vitals.modifiers, ModifierKind::Ritual)
        && !character_vitals.modifiers.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(&character_vitals.modifiers, ModifierKind::Ritual);
        effects.push(Effect::ModifierGain {
            target: ActorId::Character,
            kind: ModifierKind::Strength,
            stacks,
        });
    }

    effects.push(Effect::CardDiscardAll);
    effects.push(Effect::ModifierSetNotNew);

    for i in 0..monsters.len() {
        let mi = ActorId::Monster(i as u8);
        effects.push(Effect::TurnStart { actor: mi });

        if let Some(move_idx) = monsters[i].move_current {
            let move_effects =
                instantiate_templates(monsters[i].moves[move_idx].effects, mi, card_target, monsters);
            effects.extend(move_effects);
        }

        effects.push(Effect::MoveUpdate { monster_idx: i as u8 });
        effects.push(Effect::TurnEnd { actor: mi });
    }

    effects.push(Effect::TurnStart { actor: ActorId::Character });

    if modifier_has(&character_vitals.modifiers, ModifierKind::Burst) {
        modifier_remove(&mut character_vitals.modifiers, ModifierKind::Burst);
    }

    ProcessEffectResult::Continue {
        top: effects,
        bot: Vec::new(),
    }
}
