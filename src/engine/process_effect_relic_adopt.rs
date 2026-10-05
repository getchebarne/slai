use rand::Rng;

use crate::cards::get_card;
use crate::consts::POTION_SLOTS_MAX;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicExclusion;
use crate::effect::RewardRollTrigger;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::egg_upgrades_kind;
use crate::relics::get_relic;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::EventName;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::reward_reset;
use crate::utils::card_is_upgradable;
use crate::utils::draw_relic_excluding;
use crate::utils::has_relic;
use crate::utils::increase_max_hp;
use crate::utils::push_entity;

pub fn process_effect_relic_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_relic = id_target.expect("RelicAdopt requires id_target");
    let name = state.entities[id_relic].relic_name;

    // A second Circlet only counts on the one already held
    if name == RelicName::Circlet
        && let Some(id_held) = state.id_relics[RelicName::Circlet as usize]
    {
        state.entities[id_held].relic_counter += 1;
        return;
    }

    // Flag Relic as owned and stamp its acquisition order
    state.id_relics[name as usize] = Some(id_relic);
    state.entities[id_relic].relic_seq = state.relic_seq_next;
    state.relic_seq_next += 1;

    // Queue the Relic's pickup effects
    queue_pickup_effects(state, id_relic);
}

fn queue_pickup_effects(state: &mut GameState, id_relic: usize) {
    let id_character = state.id_character;
    let name = state.entities[id_relic].relic_name;

    // Pickup effects execute in slice order (push_front reverses)
    for &effect in state.entities[id_relic]
        .relic_effects_on_pickup
        .iter()
        .rev()
    {
        state.effect_queue.push_front(effect);
    }

    match name {
        // Potion Belt: gain 2 Potion slots
        RelicName::PotionBelt => {
            state.potion_slots_max = (state.potion_slots_max + 2).min(POTION_SLOTS_MAX as u8);
        }

        // War Paint / Whetstone: upgrade 2 random Skills / Attacks
        RelicName::WarPaint => upgrade_random_cards(state, 2, Some(CardKind::Skill)),
        RelicName::Whetstone => upgrade_random_cards(state, 2, Some(CardKind::Attack)),

        // An Egg picked up in a Shop upgrades the matching Cards still for sale
        RelicName::EggFrozen | RelicName::EggMolten | RelicName::EggToxic if state.shop.active => {
            for &(id_card, _) in state.shop.id_cards_price.iter() {
                let card = state.entities[id_card];
                if !card.card_upgraded && egg_upgrades_kind(card.card_kind, &state.id_relics) {
                    state.entities[id_card] = get_card(card.card_name, true);
                }
            }
        }

        // Pandora's Box: every starter Strike / Defend becomes a random Card
        RelicName::PandorasBox => {
            for &id in &state.id_card_deck {
                if matches!(
                    state.entities[id].card_name,
                    CardName::Strike | CardName::Defend
                ) {
                    state.effect_queue.push_front(Effect {
                        kind: EffectKind::CardTransform { upgraded: false },
                        id_source: None,
                        target: Target::Direct(Some(id)),
                    });
                }
            }
        }

        // Calling Bell: gain Curse of the Bell plus a Common, an Uncommon, and a Rare Relic
        RelicName::CallingBell => {
            // The bell arrives from a Reward context or Neow's consumed blessing
            assert!(
                state.reward.active
                    || (state.event.active
                        && matches!(state.event.name, EventName::Neow)
                        && state.event.consumed),
                "Calling Bell adopts from a Reward context or Neow"
            );

            // One screenless Relic per rarity
            let mut id_relics = Vec::with_capacity(3);
            for tier in [RelicTier::Common, RelicTier::Uncommon, RelicTier::Rare] {
                let name = draw_relic_excluding(state, tier, RelicExclusion::Screenless);
                if !has_relic(&state.id_relics, name) {
                    id_relics.push(push_entity(&mut state.entities, get_relic(name)));
                }
            }

            // The staged offer replaces the context it adopted from: a live
            // Reward loses its remaining contents; Neow's blessing closes
            if !state.reward.active {
                state.event.active = false;
            }
            reward_reset(&mut state.reward);
            state.reward.id_relics = id_relics;
            state.reward.active = true;
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::CurseOfTheBell,
                    pile: CardPile::Deck,
                    count: 1,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Tiny House: upgrade 1 random Card, +5 max HP (healed), 50 gold, 1 random Potion
        RelicName::TinyHouse => {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::RewardRollPotions {
                    count: 1,
                    trigger: RewardRollTrigger::TinyHouse,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            state.effect_queue.push_front(Effect {
                kind: EffectKind::RewardRollGold {
                    amount: Amount::Absolute(50),
                },
                id_source: None,
                target: Target::Direct(None),
            });
            increase_max_hp(state, id_character, 5);
            upgrade_random_cards(state, 1, None);
        }

        // Ring of the Serpent: replaces the starter; RingOfTheSnake's combat-start draw is lost
        RelicName::RingOfTheSerpent => {
            state.id_relics[RelicName::RingOfTheSnake as usize] = None;
        }

        _ => {}
    }
}

// Upgrade `count` random upgradable Cards, optionally kind-filtered; without replacement
fn upgrade_random_cards(state: &mut GameState, count: usize, kind: Option<CardKind>) {
    let mut ids_valid: Vec<usize> = state
        .id_card_deck
        .iter()
        .copied()
        .filter(|&id| {
            let card = &state.entities[id];
            card_is_upgradable(card) && kind.is_none_or(|card_kind| card.card_kind == card_kind)
        })
        .collect();

    for _ in 0..count.min(ids_valid.len()) {
        // Without replacement
        let idx = state.rng.random_range(0..ids_valid.len());
        let id = ids_valid.swap_remove(idx);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Direct(Some(id)),
        });
    }
}
