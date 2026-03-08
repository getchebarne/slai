use rand::Rng;

use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::engine::instantiate_templates;
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
        return ProcessEffectResult::AddAndContinue {
            top: vec![Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks,
                },
                source: None,
                target: Some(actor),
            }],
            bot: Vec::new(),
        };
    }
    ProcessEffectResult::Continue
}

pub fn process_effect_turn_end_character(
    entities: &[Entity],
    hand: &[EntityId],
    card_target: Option<EntityId>,
    alive_monsters: &[EntityId],
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    let (_, character_modifiers) = entities[0].kind.combatant_ref();

    let mut effects = Vec::new();

    if modifier_has(character_modifiers, ModifierKind::Ritual)
        && !character_modifiers.is_new[ModifierKind::Ritual as usize]
    {
        let stacks = modifier_stacks(character_modifiers, ModifierKind::Ritual);
        effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            source: None,
            target: Some(EntityId(0)),
        });
    }

    for &card_id in hand {
        effects.push(Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Some(card_id),
        });
    }
    effects.push(Effect {
        kind: EffectKind::ModifierSetNotNew,
        source: None,
        target: None,
    });

    for &mid in alive_monsters {
        let m = entities[mid.0 as usize].kind.monster_ref();
        effects.push(Effect {
            kind: EffectKind::TurnStart,
            source: None,
            target: Some(mid),
        });

        if let Some(move_idx) = m.move_current {
            let move_effects = instantiate_templates(
                m.moves[move_idx].effects,
                mid,
                &[],
                card_target,
                alive_monsters,
                rng,
            );
            effects.extend(move_effects);
        }

        effects.push(Effect {
            kind: EffectKind::MoveUpdate,
            source: None,
            target: Some(mid),
        });
        effects.push(Effect {
            kind: EffectKind::TurnEnd,
            source: None,
            target: Some(mid),
        });
    }

    effects.push(Effect {
        kind: EffectKind::TurnStart,
        source: None,
        target: Some(EntityId(0)),
    });

    if modifier_has(character_modifiers, ModifierKind::Burst) {
        effects.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Burst,
            },
            source: None,
            target: Some(EntityId(0)),
        });
    }

    ProcessEffectResult::AddAndContinue {
        top: effects,
        bot: Vec::new(),
    }
}
