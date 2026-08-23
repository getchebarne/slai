use rand::Rng;
use strum::EnumCount;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::card_template;
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
use crate::consts::GOLD_BOSS_MAX;
use crate::consts::GOLD_BOSS_MIN;
use crate::consts::MAX_CARD_REWARD_ROLL;
use crate::consts::MAX_MONSTERS;
use crate::consts::MAX_SIZE_HAND;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::entity::PlayRestriction;
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
use crate::types::Combat;
use crate::types::DeltaSign;
use crate::types::Focus;
use crate::types::RelicName;
use crate::types::RelicTier;

// Pop effect_buf back-to-front so effects pop in push order
pub fn flush_effects_from_buf_to_queue_front(state: &mut GameState) {
    while let Some(e) = state.effect_buf.pop() {
        state.effect_queue.push_front(e);
    }
}

// The focused context: reward > combat > room context > map. Derived from
// the active flags, never stored
pub fn context_focus(state: &GameState) -> Focus {
    if state.reward.active {
        Focus::Reward
    } else if state.combat.active {
        Focus::Combat
    } else if state.shop.active {
        Focus::Shop
    } else if state.chest.active {
        Focus::Chest
    } else if state.rest_site.active {
        Focus::RestSite
    } else if state.event.active {
        Focus::Event
    } else {
        Focus::Map
    }
}

// Untargeted tail-queue, shared by the reward recipes (combat_end, chest_open)
pub fn queue_effect_untargeted(state: &mut GameState, kind: EffectKind) {
    state.effect_queue.push_back(Effect {
        kind,
        id_source: None,
        target: Target::Direct(None),
    });
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
    if entity.kind != EntityKind::Card || entity.card_upgraded {
        return false;
    }
    !matches!(entity.card_kind, CardKind::Curse | CardKind::Status)
}

pub fn card_is_non_basic_non_curse(entity: &Entity) -> bool {
    entity.kind == EntityKind::Card
        && entity.card_rarity != CardRarity::Basic
        && entity.card_kind != CardKind::Curse
}

// Shift every DamagePhysical amount on the Card, clamped at 0 (Glass Knife, Ritual Dagger)
pub fn card_damage_delta(card: &mut Entity, delta: i16) {
    let num_effects = card.card_effects_len as usize;
    for effect in card.card_effects[..num_effects].iter_mut() {
        if let EffectKind::DamagePhysical { amount, .. } = &mut effect.kind {
            *amount = (*amount as i32 + delta as i32).clamp(0, u16::MAX as i32) as u16;
        }
    }
}

// Bound curses: never randomly obtainable, removable, or transformable
pub const fn card_name_never_obtainable(name: CardName) -> bool {
    matches!(
        name,
        CardName::AscendersBane | CardName::CurseOfTheBell | CardName::Necronomicurse
    )
}

pub fn get_card_effective_cost(
    card: &Entity,
    this_turn_discards: u8,
    this_combat_damage_instances_taken: u8,
    energy_current: u8,
) -> u8 {
    if let Some(cost_override) = card.card_cost_override {
        return cost_override.amount;
    }
    match card.card_cost_kind {
        CardCostKind::Fixed => card.card_cost,
        CardCostKind::MinusDiscardsThisTurn => card.card_cost.saturating_sub(this_turn_discards),
        CardCostKind::GrowsOnDamageInstanceTaken => card
            .card_cost
            .saturating_add(this_combat_damage_instances_taken),
        CardCostKind::XCost { .. } => energy_current,
    }
}

// Evaluate a PlayRestriction against the relevant slice of game state
pub fn is_play_restriction_satisfied(
    restriction: PlayRestriction,
    card_kind: CardKind,
    id_card_draw: &[usize],
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> bool {
    match restriction {
        PlayRestriction::Always => true,
        PlayRestriction::Never => match card_kind {
            CardKind::Curse => has_relic(id_relics, RelicName::BlueCandle),
            CardKind::Status => has_relic(id_relics, RelicName::MedicalKit),
            _ => false,
        },
        PlayRestriction::DrawPileEmpty => id_card_draw.is_empty(),
    }
}

// A play needs a picked monster iff any effect resolves against the pick
pub fn effects_require_target(effects: &[Effect]) -> bool {
    effects.iter().any(|effect| {
        matches!(
            effect.target,
            Target::Resolve {
                filter: CandidateFilter::Picked,
                ..
            }
        )
    })
}

pub fn entity_requires_target(entity: &Entity) -> bool {
    effects_require_target(&entity.card_effects[..entity.card_effects_len as usize])
        || effects_require_target(entity.potion_effects)
}

pub fn card_is_purgeable(entity: &Entity) -> bool {
    // Bottled Cards can't be removed or transformed while bottled
    if entity.kind != EntityKind::Card || entity.card_bottled {
        return false;
    }
    !card_name_never_obtainable(entity.card_name)
}
pub use card_is_purgeable as card_is_transformable;

// Single source of truth for which candidates a Resolve admits, whatever the
// pool. Entity predicates are total over the fat Entity; Picked / NotSource
// compare `id` against the resolve context instead
pub fn candidate_matches(
    filter: CandidateFilter,
    id: usize,
    entity: &Entity,
    id_source: Option<usize>,
    id_monster_picked: Option<usize>,
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
                    .map_or(entity.card_cost, |cost_override| cost_override.amount)
                    > 0
        }
        CandidateFilter::Picked => Some(id) == id_monster_picked,
        CandidateFilter::NotSource => Some(id) != id_source,
        CandidateFilter::NotMinion => !has_modifier(&entity.modifiers, ModifierKind::Minion),
        CandidateFilter::StarterStrike => entity.card_name == CardName::Strike,
        CandidateFilter::StarterUpgradeable => {
            matches!(entity.card_name, CardName::Strike | CardName::Defend)
                && card_is_upgradable(entity)
        }
    }
}

// Vacating a roster slot frees its Stasis hostage; mirrors place_card's hand-overflow rule
pub fn release_stasis_card(
    slot: usize,
    id_card_stasis: &mut [Option<usize>; MAX_MONSTERS],
    id_card_hand: &mut Vec<usize>,
    id_card_discard: &mut Vec<usize>,
) {
    if let Some(id_card) = id_card_stasis[slot].take() {
        if id_card_hand.len() < MAX_SIZE_HAND {
            id_card_hand.push(id_card);
        } else {
            id_card_discard.push(id_card);
        }
    }
}

pub fn place_card(state: &mut GameState, id_card: usize, pile: CardPile) -> bool {
    assert!(
        context_focus(state) == Focus::Combat,
        "Combat pile placement outside combat"
    );
    let Combat {
        id_card_hand,
        id_card_draw,
        id_card_discard,
        ..
    } = &mut state.combat;

    match pile {
        // Hand overflows to discard
        CardPile::Hand => {
            if id_card_hand.len() < MAX_SIZE_HAND {
                id_card_hand.push(id_card);
            } else {
                id_card_discard.push(id_card);
                return false;
            }
        }

        // Draw inserts at a random position
        CardPile::Draw => {
            let idx = state.rng.random_range(0..=id_card_draw.len());
            id_card_draw.insert(idx, id_card);
        }

        // Discard just goes to discard
        CardPile::Discard => id_card_discard.push(id_card),

        // Deck entry goes through CardAdopt instead
        CardPile::Deck => unreachable!(),
    }
    true
}

// Remove the id from whichever combat pile holds it; played Cards are pile-less (no-op)
pub fn detach_card(combat: &mut Combat, id_card: usize) {
    let Combat {
        id_card_hand,
        id_card_draw,
        id_card_discard,
        ..
    } = combat;
    for pile in [id_card_hand, id_card_draw, id_card_discard] {
        if let Some(pos) = pile.iter().position(|&id| id == id_card) {
            pile.remove(pos);
            return;
        }
    }
}

pub fn shuffle<T>(slice: &mut [T], rng: &mut impl Rng) {
    for idx in (1..slice.len()).rev() {
        let jdx = rng.random_range(0..=idx);
        slice.swap(idx, jdx);
    }
}

// Unceasing Top: queue rest in Combat means the player is about to act; a drawable Card ends the loop
pub fn unceasing_top_fires(state: &GameState) -> bool {
    if context_focus(state) != Focus::Combat {
        return false;
    }
    let Combat {
        id_card_hand,
        id_card_draw,
        id_card_discard,
        ..
    } = &state.combat;
    has_relic(&state.id_relics, RelicName::UnceasingTop)
        && state.effect_pending.is_none()
        && id_card_hand.is_empty()
        && !(id_card_draw.is_empty() && id_card_discard.is_empty())
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

// Shared by the live block pipeline and the FFI Card snapshot
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

// Boss gold, shared by the mid-run reward roll and the final-boss direct grant
pub fn roll_boss_gold(rng: &mut impl Rng, ascension: u8) -> u16 {
    let roll = rng.random_range(GOLD_BOSS_MIN..=GOLD_BOSS_MAX);
    if ascension >= 13 {
        (roll * 3 + 2) / 4 // x0.75 rounded half-up
    } else {
        roll
    }
}

// Fraction-of-max resolution shared by HealthDelta and MaxHealthDelta
pub fn resolve_health_fraction(health_max: u16, amount: Amount, sign: DeltaSign) -> u16 {
    match amount {
        Amount::Absolute(a) => a,
        Amount::Relative {
            numerator,
            denominator,
        }
        | Amount::RelativeRounded {
            numerator,
            denominator,
        }
        | Amount::RelativeCeil {
            numerator,
            denominator,
        } => {
            let mut raw = health_max as f32 * (numerator as f32 / denominator as f32);
            match amount {
                Amount::RelativeRounded { .. } => raw += 0.5,
                Amount::RelativeCeil { .. } => raw = raw.ceil(),
                _ => {}
            }
            let raw = raw as u32;
            match sign {
                DeltaSign::Loss => raw.max(1) as u16,
                DeltaSign::Gain => raw as u16,
            }
        }
        _ => unreachable!("health amounts resolve Absolute or Relative forms"),
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
    let mut candidates = [RelicName::RingOfTheSnake; RelicName::COUNT];
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

// Roll `count` distinct Cards; pity-bumps reward_roll_offset toward rares
pub fn roll_card_rewards(
    id_character: usize,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    out: &mut Vec<usize>,
    id_relics: &[Option<usize>; RelicName::COUNT],
    count: usize,
    rare_only: bool,
) {
    let mut character_reward_roll_offset = entities[id_character].character_reward_roll_offset;
    let mut card_names_rolled: [CardName; MAX_CARD_REWARD_ROLL] =
        [CardName::Strike; MAX_CARD_REWARD_ROLL];

    // N'loth's Gift: Rares roll three times as often; the Uncommon band keeps its width
    let chance_rare = if has_relic(id_relics, RelicName::NlothsGift) {
        CARD_REWARD_ROLL_CHANCE_RARE * 3
    } else {
        CARD_REWARD_ROLL_CHANCE_RARE
    };
    let chance_uncommon =
        chance_rare + (CARD_REWARD_ROLL_CHANCE_UNCOMMON - CARD_REWARD_ROLL_CHANCE_RARE);

    out.clear();
    for _ in 0..count {
        // Roll rarity
        let (pool, rarity) = if rare_only {
            (POOL_RARE_GREEN_CARD, CardRarity::Rare)
        } else {
            let roll = rng.random_range(0i32..=99) + character_reward_roll_offset as i32;
            if roll < chance_rare {
                (POOL_RARE_GREEN_CARD, CardRarity::Rare)
            } else if roll < chance_uncommon {
                (POOL_UNCOMMON_GREEN_CARD, CardRarity::Uncommon)
            } else {
                (POOL_COMMON_GREEN_CARD, CardRarity::Common)
            }
        };

        // Pity: reset offset on Rare hit; decrement on Common (toward more rares)
        if !rare_only {
            match rarity {
                CardRarity::Rare => character_reward_roll_offset = CARD_REWARD_ROLL_OFFSET_BASE,
                CardRarity::Common => {
                    character_reward_roll_offset =
                        (character_reward_roll_offset - 1).max(CARD_REWARD_ROLL_OFFSET_MIN);
                }
                _ => {}
            }
        }

        // Roll Cards. Loop until it's unique
        let mut name = pool[rng.random_range(0..pool.len())];
        while card_names_rolled[..out.len()].contains(&name) {
            name = pool[rng.random_range(0..pool.len())];
        }
        card_names_rolled[out.len()] = name;

        // Push the rolled Card
        let card = get_card(
            name,
            // Eggs upgrade matching rewards at roll time, so the preview shows the truth
            egg_upgrades_kind(card_template(name, false).kind, id_relics),
        );
        let id_card = push_entity(entities, card);
        out.push(id_card);
    }

    entities[id_character].character_reward_roll_offset = character_reward_roll_offset;
}
