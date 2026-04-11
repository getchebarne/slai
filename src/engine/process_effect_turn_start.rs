use crate::consts::CARDS_DRAWN_PER_TURN;
use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_remove, modifier_stacks};
use crate::state::{Energy, Vitals};
use crate::types::EntityId;

pub fn process_effect_turn_start(
    vitals: &mut Vitals,
    modifiers: &mut Modifiers,
    actor: EntityId,
    character: EntityId,
    energy: &Energy,
    monster_ids: &[EntityId],
) -> ProcessEffectResult {
    let mut effects = Vec::new();

    // Resolve new block (Blur retains, NextTurnBlock adds)
    let mut new_block: u16 = 0;
    if modifier_has(modifiers, ModifierKind::Blur) {
        new_block += vitals.block;
    }
    if modifier_has(modifiers, ModifierKind::NextTurnBlock) {
        new_block += modifier_stacks(modifiers, ModifierKind::NextTurnBlock) as u16;
        modifier_remove(modifiers, ModifierKind::NextTurnBlock);
    }
    effects.push(Effect {
        kind: EffectKind::BlockSet { amount: new_block },
        source: None,
        target: Some(actor),
    });

    // Modifier / Phantasmal
    if modifier_has(modifiers, ModifierKind::Phantasmal) {
        effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DoubleDamage,
                stacks: 1,
            },
            source: None,
            target: Some(actor),
        });
    }

    // Character-only effects
    if actor == character {
        // Draw cards and restore energy
        effects.push(Effect {
            kind: EffectKind::CardDraw {
                count: CARDS_DRAWN_PER_TURN,
            },
            source: None,
            target: None,
        });
        // TODO: may need a "reset energy" effect
        let energy_gain = energy.max.saturating_sub(energy.current);
        effects.push(Effect {
            kind: EffectKind::EnergyGain {
                amount: energy_gain,
            },
            source: None,
            target: None,
        });

        // Tick all combatant modifiers
        effects.push(Effect {
            kind: EffectKind::ModifierTick,
            source: None,
            target: Some(character),
        });
        for &mid in monster_ids {
            effects.push(Effect {
                kind: EffectKind::ModifierTick,
                source: None,
                target: Some(mid),
            });
        }

        // Modifier / NextTurnEnergy
        if modifier_has(modifiers, ModifierKind::NextTurnEnergy) {
            let stacks = modifier_stacks(modifiers, ModifierKind::NextTurnEnergy);
            effects.push(Effect {
                kind: EffectKind::EnergyGain {
                    amount: stacks as u8,
                },
                source: None,
                target: None,
            });
            modifier_remove(modifiers, ModifierKind::NextTurnEnergy);
        }

        // Modifier / InfiniteBlades
        if modifier_has(modifiers, ModifierKind::InfiniteBlades) {
            let stacks = modifier_stacks(modifiers, ModifierKind::InfiniteBlades);
            effects.push(Effect {
                kind: EffectKind::AddShivs {
                    count: stacks as u8,
                },
                source: None,
                target: None,
            });
        }
    }

    // Add and continue
    ProcessEffectResult::AddAndContinue {
        top: effects,
        bot: Vec::new(),
    }
}
