use std::collections::VecDeque;

use crate::consts::CARDS_DRAWN_PER_TURN;
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::{DispatchResult, EffectBuf};
use crate::modifier::{ModifierKind, Modifiers, modifier_has, modifier_remove, modifier_stacks};
use crate::state::Energy;
use crate::types::Vitals;

pub fn process_effect_turn_start(
    vitals: &mut Vitals,
    modifiers: &mut Modifiers,
    actor: usize,
    character: usize,
    energy: &Energy,
    monster_ids: &[usize],
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let mut top = EffectBuf::new();

    // Resolve new block (Blur retains, NextTurnBlock adds)
    let mut new_block: u16 = 0;
    if modifier_has(modifiers, ModifierKind::Blur) {
        new_block += vitals.block;
    }
    if modifier_has(modifiers, ModifierKind::NextTurnBlock) {
        new_block += modifier_stacks(modifiers, ModifierKind::NextTurnBlock) as u16;
        modifier_remove(modifiers, ModifierKind::NextTurnBlock);
    }
    top.push(Effect {
        kind: EffectKind::BlockSet { amount: new_block },
        source: None,
        target: Target::Direct(Some(actor)),
    });

    // Modifier / Phantasmal
    if modifier_has(modifiers, ModifierKind::Phantasmal) {
        top.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DoubleDamage,
                stacks: 1,
            },
            source: None,
            target: Target::Direct(Some(actor)),
        });
    }

    // Character-only effects
    if actor == character {
        // Draw cards and restore energy
        top.push(Effect {
            kind: EffectKind::CardDraw {
                count: CARDS_DRAWN_PER_TURN,
            },
            source: None,
            target: Target::Direct(None),
        });
        // TODO: may need a "reset energy" effect
        let energy_gain = energy.max.saturating_sub(energy.current);
        top.push(Effect {
            kind: EffectKind::EnergyGain {
                amount: energy_gain,
            },
            source: None,
            target: Target::Direct(None),
        });

        // Tick all combatant modifiers
        top.push(Effect {
            kind: EffectKind::ModifierTick,
            source: None,
            target: Target::Direct(Some(character)),
        });
        for &mid in monster_ids {
            top.push(Effect {
                kind: EffectKind::ModifierTick,
                source: None,
                target: Target::Direct(Some(mid)),
            });
        }

        // Modifier / NextTurnEnergy
        if modifier_has(modifiers, ModifierKind::NextTurnEnergy) {
            let stacks = modifier_stacks(modifiers, ModifierKind::NextTurnEnergy);
            top.push(Effect {
                kind: EffectKind::EnergyGain {
                    amount: stacks as u8,
                },
                source: None,
                target: Target::Direct(None),
            });
            modifier_remove(modifiers, ModifierKind::NextTurnEnergy);
        }

        // Modifier / InfiniteBlades
        if modifier_has(modifiers, ModifierKind::InfiniteBlades) {
            let stacks = modifier_stacks(modifiers, ModifierKind::InfiniteBlades);
            top.push(Effect {
                kind: EffectKind::AddShivs {
                    count: stacks as u8,
                },
                source: None,
                target: Target::Direct(None),
            });
        }

        top.push(Effect::direct(EffectKind::AwaitCombatAction, None, None));
    }

    top.push_all_front(queue);
    DispatchResult::Continue
}
