use crate::effect::Effect;
use crate::engine::{ProcessEffectResult, instantiate_templates};
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_stacks};
use crate::state::{Entity, Vitals};
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
    entities: &[Entity],
    card_target: Option<EntityId>,
    alive_monsters: &[EntityId],
) -> ProcessEffectResult {
    let (_, character_modifiers) = entities[0].kind.combatant_ref();

    let mut effects = Vec::new();

    if modifier_has(character_modifiers, ModifierKind::Ritual)
        && !character_modifiers.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::Ritual);
        effects.push(Effect::ModifierGain {
            target: EntityId(0),
            kind: ModifierKind::Strength,
            stacks,
        });
    }

    effects.push(Effect::CardDiscardAll);
    effects.push(Effect::ModifierSetNotNew);

    for &mid in alive_monsters {
        let m = entities[mid.0 as usize].kind.monster_ref();
        effects.push(Effect::TurnStart { actor: mid });

        if let Some(move_idx) = m.move_current {
            let move_effects =
                instantiate_templates(m.moves[move_idx].effects, mid, card_target, alive_monsters);
            effects.extend(move_effects);
        }

        effects.push(Effect::MoveUpdate { monster: mid });
        effects.push(Effect::TurnEnd { actor: mid });
    }

    effects.push(Effect::TurnStart { actor: EntityId(0) });

    if modifier_has(character_modifiers, ModifierKind::Burst) {
        effects.push(Effect::ModifierRemove {
            target: EntityId(0),
            kind: ModifierKind::Burst,
        });
    }

    ProcessEffectResult::Continue {
        top: effects,
        bot: Vec::new(),
    }
}
