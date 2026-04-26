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
    id_actor: usize,
    id_character: usize,
    energy: &Energy,
    id_monsters: &[usize],
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    // Stack locals
    let mut buf_effects = EffectBuf::new();

    // Poison ticks at the START of each actor's turn (StS rule). Fires for
    // both character and monsters. Pushed first so HealthLoss resolves
    // before any other turn-start effects.
    if modifier_has(modifiers, ModifierKind::Poison) {
        buf_effects.push(Effect::direct(EffectKind::PoisonTick, None, Some(id_actor)));
    }

    // Resolve new block (Blur retains, NextTurnBlock adds)
    let mut new_block: u16 = 0;
    if modifier_has(modifiers, ModifierKind::Blur) {
        new_block += vitals.block;
    }
    if modifier_has(modifiers, ModifierKind::NextTurnBlock) {
        new_block += modifier_stacks(modifiers, ModifierKind::NextTurnBlock) as u16;
        modifier_remove(modifiers, ModifierKind::NextTurnBlock);
    }
    buf_effects.push(Effect {
        kind: EffectKind::BlockSet { amount: new_block },
        id_source: None,
        target: Target::Direct(Some(id_actor)),
    });

    // Modifier / Phantasmal
    if modifier_has(modifiers, ModifierKind::Phantasmal) {
        buf_effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DoubleDamage,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    // Character-only effects
    if id_actor == id_character {
        // Draw cards and restore energy
        buf_effects.push(Effect {
            kind: EffectKind::CardDraw {
                count: CARDS_DRAWN_PER_TURN,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        // TODO: may need a "reset energy" effect
        let energy_gain = energy.max.saturating_sub(energy.current);
        buf_effects.push(Effect {
            kind: EffectKind::EnergyGain {
                amount: energy_gain,
            },
            id_source: None,
            target: Target::Direct(None),
        });

        // Tick all combatant modifiers
        buf_effects.push(Effect {
            kind: EffectKind::ModifierTick,
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
        for &id_monster in id_monsters {
            buf_effects.push(Effect {
                kind: EffectKind::ModifierTick,
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }

        // NoxiousFumes: apply Poison stacks to every alive monster at character's turn start.
        if modifier_has(modifiers, ModifierKind::NoxiousFumes) {
            let stacks = modifier_stacks(modifiers, ModifierKind::NoxiousFumes);
            for &id_monster in id_monsters {
                buf_effects.push(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Poison,
                        stacks,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }

        // Modifier / NextTurnEnergy
        if modifier_has(modifiers, ModifierKind::NextTurnEnergy) {
            let stacks = modifier_stacks(modifiers, ModifierKind::NextTurnEnergy);
            buf_effects.push(Effect {
                kind: EffectKind::EnergyGain {
                    amount: stacks as u8,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            modifier_remove(modifiers, ModifierKind::NextTurnEnergy);
        }

        // Modifier / InfiniteBlades
        if modifier_has(modifiers, ModifierKind::InfiniteBlades) {
            let stacks = modifier_stacks(modifiers, ModifierKind::InfiniteBlades);
            buf_effects.push(Effect {
                kind: EffectKind::ShivAdd {
                    count: stacks as u8,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
    }

    buf_effects.push_all_front(queue);
    DispatchResult::Continue
}
