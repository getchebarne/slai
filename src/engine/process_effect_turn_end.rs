use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_stacks};
use crate::state::{Entity, EntityKind};
use crate::types::Vitals;

pub fn process_effect_turn_end_monster(
    _vitals: &mut Vitals,
    modifiers: &Modifiers,
    actor: usize,
) -> ProcessEffectResult {
    // Modifier / Ritual (skip if newly applied)
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
                target: Target::Direct(Some(actor)),
            }],
            bot: Vec::new(),
        };
    }
    ProcessEffectResult::Continue
}

pub fn process_effect_turn_end_character(
    character: usize,
    entities: &[Entity],
    hand: &[usize],
    _card_target: Option<usize>,
    alive_monsters: &[usize],
    _rng: &mut impl Rng,
) -> ProcessEffectResult {
    let EntityKind::Character(c) = &entities[character].kind else { unreachable!() };
    let character_modifiers = &c.modifiers;
    let mut effects = Vec::new();

    // Modifier / Ritual (skip if newly applied)
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
            target: Target::Direct(Some(character)),
        });
    }

    // Discard entire hand
    for &id_card in hand {
        effects.push(Effect {
            kind: EffectKind::CardDiscard,
            source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
    effects.push(Effect {
        kind: EffectKind::ModifierSetNotNew,
        source: None,
        target: Target::Direct(None),
    });

    // Queue each monster's turn: start, execute move, update move, end
    for &mid in alive_monsters {
        let EntityKind::Monster(m) = & entities[mid].kind else { unreachable!() };
        effects.push(Effect {
            kind: EffectKind::TurnStart,
            source: None,
            target: Target::Direct(Some(mid)),
        });

        if let Some(move_idx) = m.move_current {
            // Copy the move's effects with source stamped. Resolution happens
            // lazily in the dispatcher when each effect is dequeued.
            effects.extend(m.moves[move_idx].effects.iter().map(|e| Effect {
                source: Some(mid),
                ..*e
            }));
        }

        effects.push(Effect {
            kind: EffectKind::MoveUpdate,
            source: None,
            target: Target::Direct(Some(mid)),
        });
        effects.push(Effect {
            kind: EffectKind::TurnEnd,
            source: None,
            target: Target::Direct(Some(mid)),
        });
    }

    // Start character's next turn
    effects.push(Effect {
        kind: EffectKind::TurnStart,
        source: None,
        target: Target::Direct(Some(character)),
    });

    // Modifier / Burst (consume at end of turn)
    if modifier_has(character_modifiers, ModifierKind::Burst) {
        effects.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Burst,
            },
            source: None,
            target: Target::Direct(Some(character)),
        });
    }

    // Add and continue
    ProcessEffectResult::AddAndContinue {
        top: effects,
        bot: Vec::new(),
    }
}
