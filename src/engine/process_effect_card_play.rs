use std::collections::VecDeque;

use rand::Rng;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::{DispatchResult, EffectBuf};
use crate::entity::{Entity, card_effective_cost};
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::types::CardKind;

pub fn process_effect_card_play(
    id_card: usize,
    _id_card_target: Option<usize>,
    id_character: usize,
    entities: &mut [Entity],
    _hand: &[usize],
    alive_monsters: &[usize],
    this_turn_attacks_played: &mut u8,
    last_played_card: &mut Option<usize>,
    _rng: &mut impl Rng,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    // Snapshot the played card by-value (Entity is Copy) so we can release
    // the entities borrow before mutating it for the free-to-play flag.
    let card = entities[id_card];

    // Snapshot the played card id so self-referential effects (GlassKnifeDecay)
    // can find which entity to mutate.
    *last_played_card = Some(id_card);

    // Counter for SneakyStrike-style "attacks played this turn" lookups.
    // Increment before the card's effects fire so cards like Finisher can
    // see their own play in the counter (Finisher's handler then subtracts 1
    // to exclude itself
    if card.card_kind == CardKind::Attack {
        *this_turn_attacks_played = this_turn_attacks_played.saturating_add(1);
    }

    // Free-to-play flag (Setup, Distraction): zero the cost and consume.
    let cost = card_effective_cost(&card);
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
        // Move-after-play (NOT an explicit discard — see CardMoveToDiscard
        // doc; doesn't increment this_turn_discards or trigger Reflex)
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

    // ThousandCuts: power-induced damage. id_source = None so Envenom doesn't
    // proc; DamageDeal bypasses source-side Strength/Weak scaling.
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

    // Card effects. Burst (skill-only) doubles them.
    let burst =
        modifier_has(char_modifiers, ModifierKind::Burst) && card.card_kind == CardKind::Skill;
    let reps = if burst { 2 } else { 1 };
    for _ in 0..reps {
        for e in card.card_effects[..card.card_effects_len as usize].iter() {
            buf_effects.push(Effect {
                id_source: Some(id_character),
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

    // Choke: each alive monster with Choke loses `choke_stacks` HP per card
    // play. Pushed AFTER card_effects so the played card resolves first
    // (matches StS — onUseCard's LoseHPAction is queued after the card's own
    // actions via addToBot).
    for &id_monster in alive_monsters {
        let monster_mods = &entities[id_monster].modifiers;
        if modifier_has(monster_mods, ModifierKind::Choke) {
            let stacks = modifier_stacks(monster_mods, ModifierKind::Choke);
            buf_effects.push(Effect {
                kind: EffectKind::HealthLoss {
                    amount: stacks as u16,
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    buf_effects.push_all_front(queue);
    DispatchResult::Continue
}
