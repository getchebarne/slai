use rand::Rng;
use strum::EnumCount;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::get_card;
use crate::consts::CARD_REWARD_ROLL_CHANCE_RARE;
use crate::consts::CARD_REWARD_ROLL_CHANCE_UNCOMMON;
use crate::consts::CARD_REWARD_ROLL_OFFSET_BASE;
use crate::consts::CARD_REWARD_ROLL_OFFSET_MIN;
use crate::consts::FACTOR_FRAIL;
use crate::consts::FACTOR_VULN;
use crate::consts::FACTOR_WEAK;
use crate::consts::FACTOR_WEAK_PAPER_KRANE;
use crate::consts::MAX_COMBAT_CARD_REWARD;
use crate::effect::CandidatePoolDeckFilter;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::relics::get_relic;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::RelicName;

// Pop effect_buf back-to-front so effects pop in push order
pub fn flush_effects_from_buf_to_queue_front(state: &mut GameState) {
    while let Some(e) = state.effect_buf.pop() {
        state.effect_queue.push_front(e);
    }
}

// Append an Entity to the arena; returns the assigned id
pub fn push_entity(entities: &mut Vec<Entity>, e: Entity) -> usize {
    let id = entities.len();
    entities.push(e);
    id
}

pub fn card_is_upgradable(entity: &Entity) -> bool {
    if entity.kind != EntityKind::Card {
        return false;
    }
    if entity.card_upgraded {
        return false;
    }
    !matches!(entity.card_kind, CardKind::Curse | CardKind::Status)
}

pub fn card_is_purgeable(entity: &Entity) -> bool {
    if entity.kind != EntityKind::Card {
        return false;
    }
    !matches!(entity.card_name, CardName::AscendersBane)
}

// Single source of truth for which deck cards a CandidatePoolDeckFilter admits
pub fn deck_filter_matches(filter: CandidatePoolDeckFilter, entity: &Entity) -> bool {
    match filter {
        CandidatePoolDeckFilter::Purgeable => card_is_purgeable(entity),
        CandidatePoolDeckFilter::Upgradeable => card_is_upgradable(entity),
        CandidatePoolDeckFilter::Any => entity.kind == EntityKind::Card,
        CandidatePoolDeckFilter::Transformable => {
            entity.kind == EntityKind::Card
                && entity.card_rarity != CardRarity::Basic
                && entity.card_kind != CardKind::Curse
        }
    }
}

pub fn shuffle<T>(slice: &mut [T], rng: &mut impl Rng) {
    for i in (1..slice.len()).rev() {
        let j = rng.random_range(0..=i);
        slice.swap(i, j);
    }
}

pub fn reshuffle_discard_into_draw(
    id_pile_draw: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    rng: &mut impl Rng,
) {
    id_pile_draw.append(id_pile_discard);
    shuffle(&mut id_pile_draw[..], rng);
}

// Shared by the live damage pipeline and the FFI intent view
pub fn scale_attack_damage(
    base: u16,
    source_str_stacks: i16,
    source_is_weak: bool,
    weak_paper_krane: bool,
    target_is_vulnerable: bool,
) -> u16 {
    let mut value = base as f32 + source_str_stacks as f32;
    if source_is_weak {
        value *= if weak_paper_krane {
            FACTOR_WEAK_PAPER_KRANE
        } else {
            FACTOR_WEAK
        };
    }
    if target_is_vulnerable {
        value *= FACTOR_VULN;
    }
    value.max(0.0) as u16
}

// Shared by the live block pipeline and the FFI card preview: card-played block scales with Dex/Frail
pub fn scale_block_gain(base: u16, dex_stacks: i16, frail: bool) -> u16 {
    let mut value = base as f32 + dex_stacks as f32;
    if frail {
        value *= FACTOR_FRAIL;
    }
    value.max(0.0) as u16
}

pub fn roll_card_reward_pool_green(roll: i32) -> (&'static [CardName], CardRarity) {
    if roll < CARD_REWARD_ROLL_CHANCE_RARE {
        (POOL_RARE_GREEN_CARD, CardRarity::Rare)
    } else if roll < CARD_REWARD_ROLL_CHANCE_UNCOMMON {
        (POOL_UNCOMMON_GREEN_CARD, CardRarity::Uncommon)
    } else {
        (POOL_COMMON_GREEN_CARD, CardRarity::Common)
    }
}

// No-op if already owned
pub fn grant_relic(
    name: RelicName,
    id_relics: &mut [Option<usize>; RelicName::COUNT],
    entities: &mut Vec<Entity>,
) {
    if id_relics[name as usize].is_some() {
        return;
    }
    let id = push_entity(entities, get_relic(name));
    id_relics[name as usize] = Some(id);
}

// Tier-by-roll with cascade to higher tiers when the rolled pool is exhausted
pub fn pick_relic_by_roll(
    roll: u8,
    th_common: u8,
    th_uncommon: u8,
    id_relics: &[Option<usize>; RelicName::COUNT],
    rng: &mut impl Rng,
) -> RelicName {
    if roll < th_common {
        pick_from_pool(POOL_COMMON_RELIC, id_relics, rng)
            .or_else(|| pick_from_pool(POOL_UNCOMMON_RELIC, id_relics, rng))
            .or_else(|| pick_from_pool(POOL_RARE_RELIC, id_relics, rng))
            .unwrap_or(RelicName::Circlet)
    } else if roll < th_uncommon {
        pick_from_pool(POOL_UNCOMMON_RELIC, id_relics, rng)
            .or_else(|| pick_from_pool(POOL_RARE_RELIC, id_relics, rng))
            .unwrap_or(RelicName::Circlet)
    } else {
        pick_from_pool(POOL_RARE_RELIC, id_relics, rng).unwrap_or(RelicName::Circlet)
    }
}

// Used by both elite combat-end and chest opening
pub fn add_relic_reward_for_roll(
    roll: u8,
    th_common: u8,
    th_uncommon: u8,
    id_relics: &[Option<usize>; RelicName::COUNT],
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> usize {
    let name = pick_relic_by_roll(roll, th_common, th_uncommon, id_relics, rng);
    push_entity(entities, get_relic(name))
}

pub fn pick_from_pool(
    pool: &[RelicName],
    id_relics: &[Option<usize>; RelicName::COUNT],
    rng: &mut impl Rng,
) -> Option<RelicName> {
    let mut candidates = [RelicName::SnakeRing; RelicName::COUNT];
    let mut n = 0;
    for &name in pool {
        if id_relics[name as usize].is_none() {
            candidates[n] = name;
            n += 1;
        }
    }
    if n == 0 {
        None
    } else {
        Some(candidates[rng.random_range(0..n)])
    }
}

// Roll MAX_COMBAT_CARD_REWARD distinct cards; pity-bumps reward_roll_offset toward rares
pub fn roll_card_rewards(
    id_character: usize,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    out: &mut Vec<usize>,
) {
    let mut character_reward_roll_offset = entities[id_character].character_reward_roll_offset;
    let mut rolled_card_names: [CardName; MAX_COMBAT_CARD_REWARD] =
        [CardName::Strike; MAX_COMBAT_CARD_REWARD];

    out.clear();
    for _ in 0..MAX_COMBAT_CARD_REWARD {
        let roll = rng.random_range(0i32..99) + character_reward_roll_offset as i32;
        let (pool, rarity) = roll_card_reward_pool_green(roll);
        // Pity: reset offset on Rare hit; decrement on Common (toward more rares)
        match rarity {
            CardRarity::Rare => character_reward_roll_offset = CARD_REWARD_ROLL_OFFSET_BASE,
            CardRarity::Common => {
                character_reward_roll_offset =
                    (character_reward_roll_offset - 1).max(CARD_REWARD_ROLL_OFFSET_MIN);
            }
            _ => {}
        }

        let mut name = pool[rng.random_range(0..pool.len())];
        while rolled_card_names[..out.len()].contains(&name) {
            name = pool[rng.random_range(0..pool.len())];
        }

        rolled_card_names[out.len()] = name;
        let card = get_card(name, false);
        let id_card = push_entity(entities, card);
        out.push(id_card);
    }

    entities[id_character].character_reward_roll_offset = character_reward_roll_offset;
}
