use rand::Rng;
use strum::EnumCount;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::get_card;
use crate::consts::CARD_REWARD_BASE_COUNT;
use crate::consts::CARD_REWARD_ROLL_CHANCE_RARE;
use crate::consts::CARD_REWARD_ROLL_CHANCE_UNCOMMON;
use crate::consts::CARD_REWARD_ROLL_OFFSET_BASE;
use crate::consts::CARD_REWARD_ROLL_OFFSET_MIN;
use crate::consts::FACTOR_FRAIL;
use crate::consts::FACTOR_VULN;
use crate::consts::FACTOR_VULN_ODD_MUSHROOM;
use crate::consts::FACTOR_WEAK;
use crate::consts::FACTOR_WEAK_PAPER_KRANE;
use crate::consts::MAX_COMBAT_CARD_REWARD;
use crate::consts::MAX_SIZE_HAND;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::relics::POOL_BOSS_RELIC;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_SHOP_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::relics::egg_upgrades_kind;
use crate::relics::get_relic;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::types::RelicTier;

// Pop effect_buf back-to-front so effects pop in push order
pub fn flush_effects_from_buf_to_queue_front(state: &mut GameState) {
    while let Some(e) = state.effect_buf.pop() {
        state.effect_queue.push_front(e);
    }
}

// The stack is never empty: `Mode::Map` is the permanent bottom frame
pub fn mode_top(mode_stack: &[Mode]) -> &Mode {
    mode_stack.last().expect("Mode stack never empty")
}

pub fn mode_top_mut(mode_stack: &mut [Mode]) -> &mut Mode {
    mode_stack.last_mut().expect("Mode stack never empty")
}

// Swap the active frame for a new one; its memory dies with it
pub fn mode_replace(mode_stack: &mut [Mode], mode: Mode) {
    *mode_top_mut(mode_stack) = mode;
}

// The MaxHealthDelta handler queues the matching heal itself
pub fn increase_max_hp(state: &mut GameState, id_character: usize, amount: u16) {
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(amount),
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
}

// Append an Entity to the arena; returns the assigned id
pub fn push_entity(entities: &mut Vec<Entity>, e: Entity) -> usize {
    let id = entities.len();
    entities.push(e);
    id
}

pub fn has_relic(id_relics: &[Option<usize>; RelicName::COUNT], name: RelicName) -> bool {
    id_relics[name as usize].is_some()
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

pub fn card_is_non_basic_non_curse(entity: &Entity) -> bool {
    entity.kind == EntityKind::Card
        && entity.card_rarity != CardRarity::Basic
        && entity.card_kind != CardKind::Curse
}

pub fn card_is_purgeable(entity: &Entity) -> bool {
    if entity.kind != EntityKind::Card {
        return false;
    }
    // Bottled Cards can't be removed or transformed while bottled
    if entity.card_bottled {
        return false;
    }
    !matches!(
        entity.card_name,
        CardName::AscendersBane | CardName::CurseOfTheBell
    )
}
use card_is_purgeable as card_is_transformable;

// Single source of truth for which Cards a CandidatePoolCardFilter admits (deck or hand pools)
// One filter for every Resolve. Entity predicates are total over the fat Entity;
// Picked / NotSource compare `id` against the resolve context instead
pub fn candidate_matches(
    filter: CandidateFilter,
    id: usize,
    entity: &Entity,
    id_source: Option<usize>,
    id_picked_monster: Option<usize>,
) -> bool {
    match filter {
        CandidateFilter::Any => true,
        CandidateFilter::Purgeable => card_is_purgeable(entity),
        CandidateFilter::Upgradeable => card_is_upgradable(entity),
        CandidateFilter::Transformable => card_is_transformable(entity),
        CandidateFilter::PurgeableCurse => {
            entity.card_kind == CardKind::Curse && card_is_purgeable(entity)
        }
        CandidateFilter::KindAttack => entity.card_kind == CardKind::Attack,
        CandidateFilter::KindSkill => entity.card_kind == CardKind::Skill,
        CandidateFilter::KindPower => entity.card_kind == CardKind::Power,
        CandidateFilter::Costed => {
            !matches!(entity.card_cost_kind, CardCostKind::XCost { .. })
                && entity.card_cost > 0
                && entity
                    .card_cost_override
                    .map_or(entity.card_cost, |o| o.amount)
                    > 0
        }
        CandidateFilter::Picked => Some(id) == id_picked_monster,
        CandidateFilter::NotSource => Some(id) != id_source,
    }
}

// Insert an entity id into a combat `pile`; Hand overflows to discard, Draw
// inserts at a random position. Deck entry goes through CardAddToDeck instead
// Returns false when a full hand rerouted the Card to the discard pile
pub fn place_card(state: &mut GameState, id_card: usize, pile: CardPile) -> bool {
    let Mode::Combat {
        id_hand,
        id_pile_draw,
        id_pile_discard,
        ..
    } = mode_top_mut(&mut state.mode_stack)
    else {
        unreachable!("Combat pile placement outside Combat mode")
    };
    match pile {
        CardPile::Hand => {
            if id_hand.len() < MAX_SIZE_HAND {
                id_hand.push(id_card);
            } else {
                id_pile_discard.push(id_card);
                return false;
            }
        }
        CardPile::Draw => {
            let idx = state.rng.random_range(0..=id_pile_draw.len());
            id_pile_draw.insert(idx, id_card);
        }
        CardPile::Discard => id_pile_discard.push(id_card),
        CardPile::Deck => unreachable!(),
    }
    true
}

// Remove the id from whichever combat pile holds it; played Cards are pile-less (no-op)
pub fn detach_card(mode: &mut Mode, id_card: usize) {
    let Mode::Combat {
        id_hand,
        id_pile_draw,
        id_pile_discard,
        ..
    } = mode
    else {
        unreachable!("detach_card outside Combat mode")
    };
    for pile in [id_hand, id_pile_draw, id_pile_discard] {
        if let Some(pos) = pile.iter().position(|&v| v == id_card) {
            pile.remove(pos);
            return;
        }
    }
}

pub fn shuffle<T>(slice: &mut [T], rng: &mut impl Rng) {
    for i in (1..slice.len()).rev() {
        let j = rng.random_range(0..=i);
        slice.swap(i, j);
    }
}

// Unceasing Top: queue rest in Combat means the player is about to act; a drawable Card ends the loop
pub fn unceasing_top_fires(state: &GameState) -> bool {
    let Mode::Combat {
        id_hand,
        id_pile_draw,
        id_pile_discard,
        ..
    } = mode_top(&state.mode_stack)
    else {
        return false;
    };
    has_relic(&state.id_relics, RelicName::UnceasingTop)
        && state.effect_pending.is_none()
        && id_hand.is_empty()
        && !(id_pile_draw.is_empty() && id_pile_discard.is_empty())
        && !has_modifier(
            &state.entities[state.id_character].modifiers,
            ModifierKind::NoDraw,
        )
}

// Shared by the live damage pipeline and the FFI intent view
pub fn weak_factor(is_weak: bool, paper_krane: bool) -> f32 {
    match (is_weak, paper_krane) {
        (false, _) => 1.0,
        (true, false) => FACTOR_WEAK,
        (true, true) => FACTOR_WEAK_PAPER_KRANE,
    }
}

// Odd Mushroom softens Vulnerable on the character only
pub fn vuln_factor(is_vulnerable: bool, odd_mushroom: bool) -> f32 {
    match (is_vulnerable, odd_mushroom) {
        (false, _) => 1.0,
        (true, false) => FACTOR_VULN,
        (true, true) => FACTOR_VULN_ODD_MUSHROOM,
    }
}

// Shared by the live damage pipeline and the FFI intent view
pub fn scale_attack_damage(
    base: u16,
    source_str_stacks: i16,
    weak_factor: f32,
    vuln_factor: f32,
) -> u16 {
    let value = (base as f32 + source_str_stacks as f32) * weak_factor * vuln_factor;
    value.max(0.0) as u16
}

// Shared by the live block pipeline and the FFI Card preview
pub fn scale_block_gain(base: u16, dex_stacks: i16, frail: bool) -> u16 {
    let mut value = base as f32 + dex_stacks as f32;
    if frail {
        value *= FACTOR_FRAIL;
    }
    value.max(0.0) as u16
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
        pick_relic_from_pool(POOL_COMMON_RELIC, id_relics, rng)
            .or_else(|| pick_relic_from_pool(POOL_UNCOMMON_RELIC, id_relics, rng))
            .or_else(|| pick_relic_from_pool(POOL_RARE_RELIC, id_relics, rng))
            .unwrap_or(RelicName::Circlet)
    } else if roll < th_uncommon {
        pick_relic_from_pool(POOL_UNCOMMON_RELIC, id_relics, rng)
            .or_else(|| pick_relic_from_pool(POOL_RARE_RELIC, id_relics, rng))
            .unwrap_or(RelicName::Circlet)
    } else {
        pick_relic_from_pool(POOL_RARE_RELIC, id_relics, rng).unwrap_or(RelicName::Circlet)
    }
}

// Fixed-tier uniform pick, cascading like pick_relic_by_roll when the pool is owned out
pub fn pick_relic_by_tier(
    tier: RelicTier,
    id_relics: &[Option<usize>; RelicName::COUNT],
    rng: &mut impl Rng,
) -> RelicName {
    match tier {
        RelicTier::Common => pick_relic_from_pool(POOL_COMMON_RELIC, id_relics, rng)
            .or_else(|| pick_relic_from_pool(POOL_UNCOMMON_RELIC, id_relics, rng))
            .or_else(|| pick_relic_from_pool(POOL_RARE_RELIC, id_relics, rng))
            .unwrap_or(RelicName::Circlet),
        RelicTier::Uncommon => pick_relic_from_pool(POOL_UNCOMMON_RELIC, id_relics, rng)
            .or_else(|| pick_relic_from_pool(POOL_RARE_RELIC, id_relics, rng))
            .unwrap_or(RelicName::Circlet),
        RelicTier::Rare => {
            pick_relic_from_pool(POOL_RARE_RELIC, id_relics, rng).unwrap_or(RelicName::Circlet)
        }
        RelicTier::Boss => {
            pick_relic_from_pool(POOL_BOSS_RELIC, id_relics, rng).unwrap_or(RelicName::Circlet)
        }
        RelicTier::Shop => {
            pick_relic_from_pool(POOL_SHOP_RELIC, id_relics, rng).unwrap_or(RelicName::Circlet)
        }
        RelicTier::Starter | RelicTier::Special => {
            unreachable!("No random grants from tier {:?}", tier)
        }
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

pub fn pick_relic_from_pool(
    pool: &[RelicName],
    id_relics: &[Option<usize>; RelicName::COUNT],
    rng: &mut impl Rng,
) -> Option<RelicName> {
    let mut candidates = [RelicName::SnakeRing; RelicName::COUNT];
    let mut num = 0;
    for &name in pool {
        if id_relics[name as usize].is_none() {
            candidates[num] = name;
            num += 1;
        }
    }
    if num == 0 {
        None
    } else {
        Some(candidates[rng.random_range(0..num)])
    }
}

// Question Card +1 and Busted Crown -2 fold over the base of 3
pub fn card_reward_count(id_relics: &[Option<usize>; RelicName::COUNT]) -> usize {
    let mut count = CARD_REWARD_BASE_COUNT;
    if has_relic(id_relics, RelicName::QuestionCard) {
        count += 1;
    }
    if has_relic(id_relics, RelicName::BustedCrown) {
        count = count.saturating_sub(2);
    }
    count
}

// Roll `count` distinct Cards (count <= MAX_COMBAT_CARD_REWARD); pity-bumps reward_roll_offset toward rares
pub fn roll_card_rewards(
    id_character: usize,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    out: &mut Vec<usize>,
    id_relics: &[Option<usize>; RelicName::COUNT],
    count: usize,
) {
    let mut character_reward_roll_offset = entities[id_character].character_reward_roll_offset;
    let mut card_names_rolled: [CardName; MAX_COMBAT_CARD_REWARD] =
        [CardName::Strike; MAX_COMBAT_CARD_REWARD];

    out.clear();
    for _ in 0..count {
        let roll = rng.random_range(0i32..=99) + character_reward_roll_offset as i32;
        let (pool, rarity) = if roll < CARD_REWARD_ROLL_CHANCE_RARE {
            (POOL_RARE_GREEN_CARD, CardRarity::Rare)
        } else if roll < CARD_REWARD_ROLL_CHANCE_UNCOMMON {
            (POOL_UNCOMMON_GREEN_CARD, CardRarity::Uncommon)
        } else {
            (POOL_COMMON_GREEN_CARD, CardRarity::Common)
        };

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
        while card_names_rolled[..out.len()].contains(&name) {
            name = pool[rng.random_range(0..pool.len())];
        }

        card_names_rolled[out.len()] = name;
        let card = get_card(
            name,
            // Eggs upgrade matching rewards at roll time, so the preview shows the truth
            egg_upgrades_kind(get_card(name, false).card_kind, id_relics),
        );
        let id_card = push_entity(entities, card);
        out.push(id_card);
    }

    entities[id_character].character_reward_roll_offset = character_reward_roll_offset;
}
