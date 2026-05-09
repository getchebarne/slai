use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::{DispatchResult, EffectBuf};
use crate::entity::{CardCostKind, Entity, card_effective_cost};
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::types::{CardKind, N_RELICS, RelicName};

pub fn process_effect_card_play(
    id_card: usize,
    id_character: usize,
    entities: &mut [Entity],
    alive_monsters: &[usize],
    this_turn_attacks_played: &mut u8,
    this_turn_discards: u8,
    this_combat_damage_instances_taken: u8,
    energy_current: u8,
    relics_active: u128,
    id_relics: &[usize; N_RELICS],
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    let card = entities[id_card];

    // Increment before effects fire so self-counting cards see their own play
    if card.card_kind == CardKind::Attack {
        *this_turn_attacks_played = this_turn_attacks_played.saturating_add(1);

        // Kunai: every 3rd attack -> +1 Dexterity
        if relics_active & (1u128 << RelicName::Kunai as u32) != 0 {
            let counter = &mut entities[id_relics[RelicName::Kunai as usize]].relic_counter;
            *counter += 1;
            if *counter >= 3 {
                *counter = 0;
                effect_queue.push_back(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Dexterity,
                        stacks: 1,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                });
            }
        }
        // Shuriken: every 3rd attack -> +1 Strength
        if relics_active & (1u128 << RelicName::Shuriken as u32) != 0 {
            let counter = &mut entities[id_relics[RelicName::Shuriken as usize]].relic_counter;
            *counter += 1;
            if *counter >= 3 {
                *counter = 0;
                effect_queue.push_back(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Strength,
                        stacks: 1,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_character)),
                });
            }
        }
    }

    let cost = card_effective_cost(
        &card,
        this_turn_discards,
        this_combat_damage_instances_taken,
        energy_current,
    );

    // X-cost reads raw energy_current (not effective_cost) so Setup-flagged X-cost still scales
    let multiplier = match card.card_cost_kind {
        CardCostKind::XCost { offset } => (energy_current as i16 + offset as i16).max(0) as usize,
        _ => 1,
    };

    if card.card_free_to_play_once {
        entities[id_card].card_free_to_play_once = false;
    }

    // Stack locals
    let mut buf_effects = EffectBuf::new();

    buf_effects.push(Effect {
        kind: EffectKind::EnergyLoss { amount: cost },
        id_source: None,
        target: Target::Direct(None),
    });

    if card.card_exhaust {
        buf_effects.push(Effect {
            kind: EffectKind::CardExhaust,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    } else if card.card_kind == CardKind::Power {
        buf_effects.push(Effect {
            kind: EffectKind::CardRemove,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    } else {
        // Not a real discard: skips this_turn_discards and Reflex
        buf_effects.push(Effect {
            kind: EffectKind::CardMoveToDiscard,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }

    let char_modifiers = &entities[id_character].modifiers;

    // AfterImage
    if modifier_has(char_modifiers, ModifierKind::AfterImage) {
        let stacks = modifier_stacks(char_modifiers, ModifierKind::AfterImage);
        buf_effects.push(Effect {
            kind: EffectKind::BlockGain {
                amount: stacks as u16,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_character)),
        });
    }

    // ThousandCuts: id_source = None to skip Envenom proc and Strength/Weak scaling
    if modifier_has(char_modifiers, ModifierKind::ThousandCuts) {
        let stacks = modifier_stacks(char_modifiers, ModifierKind::ThousandCuts);
        for &id_monster in alive_monsters {
            buf_effects.push(Effect {
                kind: EffectKind::DamageDeal {
                    amount: stacks as u16,
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // SharpHide (reflects on attack cards)
    if card.card_kind == CardKind::Attack {
        for &id_monster in alive_monsters {
            let monster_modifiers = &entities[id_monster].modifiers;
            if modifier_has(monster_modifiers, ModifierKind::SharpHide) {
                let stacks = modifier_stacks(monster_modifiers, ModifierKind::SharpHide);
                buf_effects.push(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: stacks as u16,
                    },
                    id_source: Some(id_monster),
                    target: Target::Direct(Some(id_character)),
                });
            }
        }
    }

    // Burst (skill-only) doubles; X-cost multiplies by X; the two stack multiplicatively
    let burst =
        modifier_has(char_modifiers, ModifierKind::Burst) && card.card_kind == CardKind::Skill;
    let reps = if burst { 2 * multiplier } else { multiplier };
    for _ in 0..reps {
        for e in card.card_effects[..card.card_effects_len as usize].iter() {
            buf_effects.push(Effect {
                id_source: Some(id_card),
                ..*e
            });
        }
    }
    if burst {
        buf_effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Burst,
                stacks: -1,
            },
            id_source: Some(id_character),
            target: Target::Direct(Some(id_character)),
        });
    }

    // Vigor: full clear on first Attack played
    if card.card_kind == CardKind::Attack && modifier_has(char_modifiers, ModifierKind::Vigor) {
        buf_effects.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Vigor,
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
    }

    // Choke: pushed after card_effects so the played card resolves first
    for &id_monster in alive_monsters {
        let mods_monster = &entities[id_monster].modifiers;
        if modifier_has(mods_monster, ModifierKind::Choke) {
            let stacks = modifier_stacks(mods_monster, ModifierKind::Choke);
            buf_effects.push(Effect {
                kind: EffectKind::HealthLoss {
                    amount: stacks as u16,
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // Enrage: gain strength on played skill
    if card.card_kind == CardKind::Skill {
        for &id_monster in alive_monsters {
            let mods_monster = &entities[id_monster].modifiers;
            if modifier_has(mods_monster, ModifierKind::Enrage) {
                let stacks = modifier_stacks(mods_monster, ModifierKind::Enrage);
                buf_effects.push(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Strength,
                        stacks,
                    },
                    id_source: Some(id_monster),
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }
    }

    buf_effects.push_all_front(effect_queue);
    DispatchResult::Continue
}
