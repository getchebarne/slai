use rand::Rng;

use crate::consts::CHEST_LARGE_GOLD_BASE;
use crate::consts::CHEST_LARGE_GOLD_CHANCE;
use crate::consts::CHEST_LARGE_TH_COMMON;
use crate::consts::CHEST_LARGE_TH_UNCOMMON;
use crate::consts::CHEST_MEDIUM_GOLD_BASE;
use crate::consts::CHEST_MEDIUM_GOLD_CHANCE;
use crate::consts::CHEST_MEDIUM_TH_COMMON;
use crate::consts::CHEST_MEDIUM_TH_UNCOMMON;
use crate::consts::CHEST_SMALL_GOLD_BASE;
use crate::consts::CHEST_SMALL_GOLD_CHANCE;
use crate::consts::CHEST_SMALL_TH_COMMON;
use crate::consts::CHEST_SMALL_TH_UNCOMMON;
use crate::consts::GOLD_ELITE_MAX;
use crate::consts::GOLD_ELITE_MIN;
use crate::consts::GOLD_MONSTER_MAX;
use crate::consts::GOLD_MONSTER_MIN;
use crate::consts::MATRYOSHKA_TH_COMMON;
use crate::consts::MATRYOSHKA_TH_UNCOMMON;
use crate::consts::MAX_COMBAT_CARD_REWARD;
use crate::consts::POTION_DROP_CHANCE_BASE;
use crate::consts::POTION_DROP_CHANCE_MOD_HIT;
use crate::consts::POTION_DROP_CHANCE_MOD_MAX;
use crate::consts::POTION_DROP_CHANCE_MOD_MIN;
use crate::consts::POTION_DROP_CHANCE_MOD_MISS;
use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::effect::Amount;
use crate::effect::RewardSource;
use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::relics::get_relic;
use crate::types::ChestKind;
use crate::types::Mode;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::utils::add_relic_reward_for_roll;
use crate::utils::card_reward_count;
use crate::utils::has_relic;
use crate::utils::mode_replace;
use crate::utils::mode_top;
use crate::utils::push_entity;
use crate::utils::roll_card_rewards;

#[derive(Debug, Clone, Copy)]
struct ChestParams {
    gold_chance: u8,
    gold_base: u16,
    th_common: u8,
    th_uncommon: u8,
}

pub fn process_effect_reward_roll(state: &mut GameState, source: RewardSource) {
    let mut id_card_bundles: Vec<Vec<usize>> = Vec::new();
    let mut id_relics_reward: Vec<usize> = Vec::new();
    let mut id_potions: Vec<usize> = Vec::new();
    let mut gold: Option<u16> = None;

    match source {
        // Card-only rewards rolled like a combat reward (Busted Crown applies): Dream Catcher
        // takes one bundle, Orrery five
        RewardSource::Cards { bundles } => {
            let cards_per_bundle = card_reward_count(&state.id_relics);
            for _ in 0..bundles {
                let mut id_cards: Vec<usize> = Vec::new();
                roll_card_rewards(
                    state.id_character,
                    &mut state.entities,
                    &mut state.rng,
                    &mut id_cards,
                    &state.id_relics,
                    cards_per_bundle,
                );
                id_card_bundles.push(id_cards);
            }
        }

        RewardSource::Chest { kind } => {
            let chest_params = match kind {
                ChestKind::Small => ChestParams {
                    gold_chance: CHEST_SMALL_GOLD_CHANCE,
                    gold_base: CHEST_SMALL_GOLD_BASE,
                    th_common: CHEST_SMALL_TH_COMMON,
                    th_uncommon: CHEST_SMALL_TH_UNCOMMON,
                },
                ChestKind::Medium => ChestParams {
                    gold_chance: CHEST_MEDIUM_GOLD_CHANCE,
                    gold_base: CHEST_MEDIUM_GOLD_BASE,
                    th_common: CHEST_MEDIUM_TH_COMMON,
                    th_uncommon: CHEST_MEDIUM_TH_UNCOMMON,
                },
                ChestKind::Large => ChestParams {
                    gold_chance: CHEST_LARGE_GOLD_CHANCE,
                    gold_base: CHEST_LARGE_GOLD_BASE,
                    th_common: CHEST_LARGE_TH_COMMON,
                    th_uncommon: CHEST_LARGE_TH_UNCOMMON,
                },
            };

            let roll = state.rng.random_range(0..100) as u8;
            if roll < chest_params.gold_chance {
                gold = Some(roll_gold_amount(&mut state.rng, chest_params));
            }

            // Matryoshka: the next 2 chests hold an extra Relic (75% Common / 25% Uncommon)
            if let Some(id) = state.id_relics[RelicName::Matryoshka as usize]
                && state.entities[id].relic_counter > 0
            {
                // Grab mutable Matryoshka reference
                let relic = &mut state.entities[id];

                // Decrease counter, set `relic_used_up` if appropriate
                relic.relic_counter -= 1;
                relic.relic_used_up = relic.relic_counter == 0;

                // Push extra Relic
                id_relics_reward.push(add_relic_reward_for_roll(
                    state.rng.random_range(0..100) as u8,
                    MATRYOSHKA_TH_COMMON,
                    MATRYOSHKA_TH_UNCOMMON,
                    &state.id_relics,
                    &mut state.entities,
                    &mut state.rng,
                ));
            }

            // Push Relic
            id_relics_reward.push(add_relic_reward_for_roll(
                roll,
                chest_params.th_common,
                chest_params.th_uncommon,
                &state.id_relics,
                &mut state.entities,
                &mut state.rng,
            ));
        }

        RewardSource::Combat {
            room_kind,
            escaped,
            event_gold,
            event_relic,
            event_relic_roll,
        } => {
            // Select roll parameters according to `RoomKind`
            let (gold_amount, relic_thresholds, event_relic) = match room_kind {
                RoomKind::CombatMonster => (
                    (!escaped).then_some(Amount::Range {
                        min: GOLD_MONSTER_MIN,
                        max: GOLD_MONSTER_MAX,
                    }),
                    None,
                    None,
                ),
                RoomKind::CombatElite => (
                    Some(Amount::Range {
                        min: GOLD_ELITE_MIN,
                        max: GOLD_ELITE_MAX,
                    }),
                    Some((RELIC_TIER_TH_COMMON, RELIC_TIER_TH_UNCOMMON)),
                    None,
                ),
                RoomKind::EventRoom => (
                    // Event combats inject their event-specific extras
                    Some(event_gold.expect("Event fight without stamped loot")),
                    event_relic_roll.then_some((RELIC_TIER_TH_COMMON, RELIC_TIER_TH_UNCOMMON)),
                    event_relic,
                ),
                _ => unreachable!(
                    "RewardRoll Combat with non-combat room_kind: {:?}",
                    room_kind
                ),
            };

            // Prayer Wheel: adds a second bundle on normal fights
            let bundle_count = if room_kind == RoomKind::CombatMonster
                && has_relic(&state.id_relics, RelicName::PrayerWheel)
            {
                2
            } else {
                1
            };

            // Roll Cards
            let cards_per_bundle = card_reward_count(&state.id_relics);
            for _ in 0..bundle_count {
                let mut id_cards: Vec<usize> = Vec::with_capacity(MAX_COMBAT_CARD_REWARD);
                roll_card_rewards(
                    state.id_character,
                    &mut state.entities,
                    &mut state.rng,
                    &mut id_cards,
                    &state.id_relics,
                    cards_per_bundle,
                );
                id_card_bundles.push(id_cards);
            }

            // Roll Relic (only for Elite combats)
            if let Some((th_common, th_uncommon)) = relic_thresholds {
                let id_relic_1 = add_relic_reward_for_roll(
                    state.rng.random_range(0..100) as u8,
                    th_common,
                    th_uncommon,
                    &state.id_relics,
                    &mut state.entities,
                    &mut state.rng,
                );
                id_relics_reward.push(id_relic_1);

                // Black Star: elites drop a second Relic with an independent tier roll
                if has_relic(&state.id_relics, RelicName::BlackStar) {
                    // Snapshot currently-owned Relics
                    let mut id_relic_aux = state.id_relics;

                    // Set first rolled Relic as if owned
                    id_relic_aux[state.entities[id_relic_1].relic_name as usize] = Some(id_relic_1);

                    // Roll second Relic using the auxiliary snapshot that already includes the first roll
                    id_relics_reward.push(add_relic_reward_for_roll(
                        state.rng.random_range(0..100) as u8,
                        th_common,
                        th_uncommon,
                        &id_relic_aux,
                        &mut state.entities,
                        &mut state.rng,
                    ));
                }
            }

            // Event-injected Relic; Circlet substitutes when it is already owned
            if let Some(name) = event_relic {
                let name = if has_relic(&state.id_relics, name) {
                    RelicName::Circlet
                } else {
                    name
                };
                id_relics_reward.push(push_entity(&mut state.entities, get_relic(name)));
            }

            // White Beast Statue: guaranteed drop, bypassing the drifting chance roll
            let has_white_beast_statue = has_relic(&state.id_relics, RelicName::WhiteBeastStatue);

            // Roll Potions (Sozu doesn't stop the roll: the staged Potion adopts to nothing)
            if has_white_beast_statue
                || roll_potion_drop(&mut state.rng, &mut state.potion_drop_mod)
            {
                let name = get_random_potion_name(&mut state.rng, false);
                let id = push_entity(&mut state.entities, get_potion(name));
                id_potions.push(id);
            }

            // Roll gold
            if let Some(amount) = gold_amount {
                let mut rolled = match amount {
                    Amount::Absolute(amount) => amount,
                    Amount::Range { min, max } => state.rng.random_range(min..=max),
                    _ => unreachable!("Reward gold only resolves Absolute or Range"),
                };
                // Golden Idol: 25% bonus rounded half-up on combat rewards only
                if has_relic(&state.id_relics, RelicName::GoldenIdol) {
                    rolled += (rolled + 2) / 4;
                }
                gold = Some(rolled);
            }
        }

        // Stage `count` rolled Potions on the reward screen (The Lab, The Woman in Blue)
        RewardSource::Potions { count } => {
            for _ in 0..count {
                let potion_name = get_random_potion_name(&mut state.rng, false);
                let id = push_entity(&mut state.entities, get_potion(potion_name));
                id_potions.push(id);
            }
        }
    }

    let mode_reward = Mode::Reward {
        reward_id_cards: id_card_bundles,
        reward_id_relics: id_relics_reward,
        reward_id_potions: id_potions,
        reward_gold: gold,
    };

    // Shop rolls (Orrery, Cauldron) overlay the stock; every other source replaces its own frame
    if matches!(mode_top(&state.mode_stack), Mode::Shop { .. }) {
        state.mode_stack.push(mode_reward);
    } else {
        mode_replace(&mut state.mode_stack, mode_reward);
    }
}

fn roll_gold_amount(rng: &mut impl Rng, chest_params: ChestParams) -> u16 {
    let base = chest_params.gold_base as f32;
    let factor = rng.random_range(0.9..=1.1);
    (base * factor).round() as u16
}

// +10 on miss, -10 on hit; clamps to [-30, +60] ([10%, 100%])
fn roll_potion_drop(rng: &mut impl Rng, potion_drop_mod: &mut i8) -> bool {
    let roll = rng.random_range(0..100) as u8;
    let chance = (POTION_DROP_CHANCE_BASE as i16 + *potion_drop_mod as i16).clamp(0, 100) as u8;

    if roll < chance {
        *potion_drop_mod = (*potion_drop_mod + POTION_DROP_CHANCE_MOD_HIT)
            .clamp(POTION_DROP_CHANCE_MOD_MIN, POTION_DROP_CHANCE_MOD_MAX);
        true
    } else {
        *potion_drop_mod = (*potion_drop_mod + POTION_DROP_CHANCE_MOD_MISS)
            .clamp(POTION_DROP_CHANCE_MOD_MIN, POTION_DROP_CHANCE_MOD_MAX);
        false
    }
}
