use rand::Rng;

use crate::consts::MAX_SIZE_DECK;
use crate::consts::POTION_SLOTS_MAX;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::relics::get_relic;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::EventName;
use crate::types::RelicName;
use crate::types::reward_reset;
use crate::utils::card_is_upgradable;
use crate::utils::increase_max_hp;
use crate::utils::pick_relic_from_pool;
use crate::utils::push_entity;

pub fn process_effect_relic_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_relic = id_target.expect("RelicAdopt requires id_target");

    // Flag Relic as owned and stamp its acquisition order
    let name = state.entities[id_relic].relic_name;
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
    for &eff in state.entities[id_relic]
        .relic_effects_on_pickup
        .iter()
        .rev()
    {
        state.effect_queue.push_front(eff);
    }

    match name {
        // Potion Belt: gain 2 Potion slots
        RelicName::PotionBelt => {
            state.potion_slots_max = (state.potion_slots_max + 2).min(POTION_SLOTS_MAX as u8);
        }

        // War Paint / Whetstone: upgrade 2 random Skills / Attacks
        RelicName::WarPaint => upgrade_random_cards(state, 2, Some(CardKind::Skill)),
        RelicName::Whetstone => upgrade_random_cards(state, 2, Some(CardKind::Attack)),

        // Pandora's Box: every starter Strike / Defend becomes a random Card
        RelicName::PandorasBox => {
            let mut id_starter = [0usize; MAX_SIZE_DECK];
            let mut id_starter_num = 0;
            for &id in &state.id_card_deck {
                if matches!(
                    state.entities[id].card_name,
                    CardName::Strike | CardName::Defend
                ) {
                    id_starter[id_starter_num] = id;
                    id_starter_num += 1;
                }
            }
            for &id in &id_starter[..id_starter_num] {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardTransform { upgraded: false },
                    id_source: None,
                    target: Target::Direct(Some(id)),
                });
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

            // Roll one Relic for each rarity
            let mut id_relics = Vec::with_capacity(3);
            for pool in [POOL_COMMON_RELIC, POOL_UNCOMMON_RELIC, POOL_RARE_RELIC] {
                if let Some(name) = pick_relic_from_pool(pool, &state.id_relics, &mut state.rng) {
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
                kind: EffectKind::PotionAddRandom { limited: false },
                id_source: None,
                target: Target::Direct(None),
            });
            state.effect_queue.push_front(Effect {
                kind: EffectKind::GoldDelta {
                    sign: DeltaSign::Gain,
                    amount: Amount::Absolute(50),
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
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
    let mut ids_valid = [0usize; MAX_SIZE_DECK];
    let mut num = 0;
    for &id in &state.id_card_deck {
        let card = &state.entities[id];
        if card_is_upgradable(card) && kind.is_none_or(|card_kind| card.card_kind == card_kind) {
            ids_valid[num] = id;
            num += 1;
        }
    }

    for _ in 0..count.min(num) {
        let idx = state.rng.random_range(0..num);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Direct(Some(ids_valid[idx])),
        });

        // Without replacement
        ids_valid[idx] = ids_valid[num - 1];
        num -= 1;
    }
}
