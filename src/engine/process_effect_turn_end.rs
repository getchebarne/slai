use crate::effect::Effect;
use crate::engine::{ProcessEffectResult, instantiate_templates};
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_remove, modifier_stacks};
use crate::monsters::Monster;
use crate::state::Vitals;
use crate::types::EntityId;

pub fn process_effect_turn_end_monster(
    _vitals: &mut Vitals,
    modifiers: &Modifiers,
    actor: EntityId,
) -> ProcessEffectResult {
    if modifier_has(modifiers, ModifierKind::Ritual)
        && !modifiers.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(modifiers, ModifierKind::Ritual);
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
    _character_vitals: &mut Vitals,
    character_modifiers: &mut Modifiers,
    monsters: &[Monster],
    card_target: Option<EntityId>,
    character_id: EntityId,
) -> ProcessEffectResult {
    let mut effects = Vec::new();

    if modifier_has(character_modifiers, ModifierKind::Ritual)
        && !character_modifiers.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::Ritual);
        effects.push(Effect::ModifierGain {
            target: character_id,
            kind: ModifierKind::Strength,
            stacks,
        });
    }

    effects.push(Effect::CardDiscardAll);
    effects.push(Effect::ModifierSetNotNew);

    for monster in monsters.iter() {
        let mid = monster.id;
        effects.push(Effect::TurnStart { actor: mid });

        if let Some(move_idx) = monster.move_current {
            let move_effects =
                instantiate_templates(monster.moves[move_idx].effects, mid, card_target, character_id, monsters);
            effects.extend(move_effects);
        }

        effects.push(Effect::MoveUpdate { monster: mid });
        effects.push(Effect::TurnEnd { actor: mid });
    }

    effects.push(Effect::TurnStart { actor: character_id });

    if modifier_has(character_modifiers, ModifierKind::Burst) {
        modifier_remove(character_modifiers, ModifierKind::Burst);
    }

    ProcessEffectResult::Continue {
        top: effects,
        bot: Vec::new(),
    }
}
