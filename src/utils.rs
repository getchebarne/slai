use std::collections::VecDeque;

use rand::Rng;
use strum::EnumCount;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_COLORLESS_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_COLORLESS_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::get_card;
use crate::cards::get_card_template;
use crate::consts::ASCENSION_CARD_UPGRADE_CUT_LEVEL;
use crate::consts::CARD_REWARD_BASE_COUNT;
use crate::consts::CARD_REWARD_ROLL_CHANCE_RARE;
use crate::consts::CARD_REWARD_ROLL_CHANCE_RARE_ELITE;
use crate::consts::CARD_REWARD_ROLL_CHANCE_UNCOMMON;
use crate::consts::CARD_REWARD_ROLL_CHANCE_UNCOMMON_ELITE;
use crate::consts::CARD_REWARD_ROLL_OFFSET_BASE;
use crate::consts::CARD_REWARD_ROLL_OFFSET_MIN;
use crate::consts::CARD_REWARD_UPGRADE_CHANCE_ACT2;
use crate::consts::CARD_REWARD_UPGRADE_CHANCE_ACT2_A12;
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
use crate::consts::NEOW_UNCOMMON_CHANCE;
use crate::consts::SHOP_CARD_CUT_RARE;
use crate::consts::SHOP_CARD_CUT_UNCOMMON;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicExclusion;
use crate::effect::RewardRollTrigger;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::CostOverride;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::entity::PlayRestriction;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::relics::egg_upgrades_kind;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::Combat;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::Focus;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RoomKind;

// Pop effect_buf back-to-front so effects pop in push order
pub fn flush_effects_from_buf_to_queue_front(state: &mut GameState) {
    while let Some(e) = state.effect_buf.pop() {
        state.effect_queue.push_front(e);
    }
}

// The focused context: reward > combat > Room context > map. Derived from
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

// Untargeted tail-queue, shared by the reward recipes
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

// Source -> actor: Cards delegate to Character; Monsters/Character self
pub fn get_id_actor(entities: &[Entity], id_character: usize, id_source: usize) -> usize {
    if entities[id_source].kind == EntityKind::Card {
        id_character
    } else {
        id_source
    }
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
pub const fn card_name_bound_curse(name: CardName) -> bool {
    matches!(
        name,
        CardName::AscendersBane | CardName::CurseOfTheBell | CardName::Necronomicurse
    )
}

// Skipped by the in-combat random draws, but still shop stock
pub const fn card_name_healing(name: CardName) -> bool {
    matches!(
        name,
        CardName::Alchemize | CardName::BandageUp | CardName::Bite
    )
}

// Normality in hand caps the turn at 3 plays; Velvet Choker at 6
pub fn play_cap_reached(
    id_card_hand: &[usize],
    entities: &[Entity],
    id_relics: &[Option<usize>; RelicName::COUNT],
    this_turn_cards_played: u8,
) -> bool {
    let normality = this_turn_cards_played >= 3
        && id_card_hand
            .iter()
            .any(|&id| entities[id].card_name == CardName::Normality);
    let choker = this_turn_cards_played >= 6 && has_relic(id_relics, RelicName::VelvetChoker);
    normality || choker
}

// The Card's cost this turn: its per-turn override, else its combat cost
pub fn get_card_cost_this_turn(card: &Entity) -> u8 {
    card.card_cost_override
        .map_or(card.card_cost, |cost_override| cost_override.amount)
}

// The energy a play spends: none when free to play once, all of it for X-cost
pub fn get_card_effective_cost(card: &Entity, energy_current: u16) -> u16 {
    if card.card_free_to_play_once {
        return 0;
    }
    match card.card_cost_kind {
        CardCostKind::Fixed
        | CardCostKind::MinusDiscardsThisTurn
        | CardCostKind::GrowsOnDamageInstanceTaken => get_card_cost_this_turn(card) as u16,
        CardCostKind::XCost { .. } => energy_current,
    }
}

pub fn cards_grow_on_damage(state: &mut GameState) {
    for pile in [
        &state.combat.id_card_hand,
        &state.combat.id_card_draw,
        &state.combat.id_card_discard,
    ] {
        for &id_card in pile.iter() {
            let card = &mut state.entities[id_card];
            if !matches!(
                card.card_cost_kind,
                CardCostKind::GrowsOnDamageInstanceTaken
            ) {
                continue;
            }
            card.card_cost = card.card_cost.saturating_add(1);

            // A per-turn override grows with the Card
            if let Some(CostOverride {
                amount,
                scope: CostScope::Turn,
            }) = &mut card.card_cost_override
            {
                *amount = amount.saturating_add(1);
            }
        }
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

// A play needs a picked Monster iff any effect resolves against the pick
pub fn effects_require_target(effects: &[Effect]) -> bool {
    effects.iter().any(|effect| {
        matches!(
            effect.target,
            Target::Resolve {
                candidate_pool: CandidatePool::MonsterPicked,
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
    !card_name_bound_curse(entity.card_name)
}
pub use card_is_purgeable as card_is_transformable;

// Single source of truth for which candidates a Resolve admits, whatever the
// pool. Entity predicates are total over the fat Entity; Picked / NotSource
// compare `id` against the resolve context instead
// One filter pass over the whole candidate set; Costed and NotSourceUnlessAlone fall back
// on what else survives, which no single-entity test can express
pub fn filter_candidates(
    filter: CandidateFilter,
    candidates: &mut Vec<usize>,
    entities: &[Entity],
    id_source: Option<usize>,
) {
    match filter {
        // Printed cost is only the fallback tier when no Card has a live one
        CandidateFilter::Costed => {
            let live = |entity: &Entity| {
                !matches!(entity.card_cost_kind, CardCostKind::XCost { .. })
                    && get_card_cost_this_turn(entity) > 0
            };
            let printed = |entity: &Entity| {
                !matches!(entity.card_cost_kind, CardCostKind::XCost { .. }) && entity.card_cost > 0
            };
            if candidates.iter().any(|&id| live(&entities[id])) {
                candidates.retain(|&id| live(&entities[id]));
            } else {
                candidates.retain(|&id| printed(&entities[id]));
            }
        }
        CandidateFilter::NotSource => candidates.retain(|&id| Some(id) != id_source),
        // The last Monster standing falls back to targeting itself
        CandidateFilter::NotSourceUnlessAlone => {
            candidates.retain(|&id| Some(id) != id_source);
            if candidates.is_empty()
                && let Some(id_source) = id_source
            {
                candidates.push(id_source);
            }
        }
        _ => candidates.retain(|&id| entity_matches(filter, &entities[id])),
    }
}

fn entity_matches(filter: CandidateFilter, entity: &Entity) -> bool {
    match filter {
        CandidateFilter::Any => true,
        CandidateFilter::Purgeable => card_is_purgeable(entity),
        // Astrolabe, Empty Cage and Drug Dealer may take bottled Cards; only the bound curses are off-limits
        CandidateFilter::NotBoundCurse => {
            entity.kind == EntityKind::Card && !card_name_bound_curse(entity.card_name)
        }
        CandidateFilter::Upgradeable => card_is_upgradable(entity),
        CandidateFilter::Transformable => card_is_transformable(entity),
        CandidateFilter::PurgeableCurse => {
            entity.card_kind == CardKind::Curse && card_is_purgeable(entity)
        }
        CandidateFilter::KindAttack => entity.card_kind == CardKind::Attack,
        CandidateFilter::KindSkill => entity.card_kind == CardKind::Skill,
        CandidateFilter::KindPower => entity.card_kind == CardKind::Power,
        CandidateFilter::Costed
        | CandidateFilter::NotSource
        | CandidateFilter::NotSourceUnlessAlone => {
            unreachable!("{filter:?} is set-level; filter_candidates handles it")
        }
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
    entities: &[Entity],
    effect_queue: &mut VecDeque<Effect>,
) {
    if let Some(id_card) = id_card_stasis[slot].take() {
        if id_card_hand.len() < MAX_SIZE_HAND {
            id_card_hand.push(id_card);
        } else {
            id_card_discard.push(id_card);
        }

        // A returned Eviscerate restarts its cost this turn at its combat cost less this turn's discards
        if entities[id_card].card_cost_kind == CardCostKind::MinusDiscardsThisTurn {
            effect_queue.push_front(Effect {
                kind: EffectKind::CardCostMinusDiscards,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
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
            let idx = if id_card_draw.is_empty() {
                0
            } else {
                state.rng.random_range(0..id_card_draw.len())
            };
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

// Odd Mushroom softens Vulnerable on the Character only
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
    double_damage: bool,
    pen_nib: bool,
    weak_factor: f32,
    vuln_factor: f32,
    flight: bool,
) -> u16 {
    let mut value = base as f32 + source_str_stacks as f32;
    if double_damage {
        value *= 2.0;
    }
    if pen_nib {
        value *= 2.0;
    }
    value *= weak_factor * vuln_factor;
    if flight {
        value *= 0.5;
    }
    value.max(0.0) as u16
}

// Shared by the live block pipeline and the FFI Card snapshot
pub fn scale_block_gain(base: u16, dex_stacks: i16, is_frail: bool) -> u16 {
    let mut value = base as f32 + dex_stacks as f32;
    if is_frail {
        value *= FACTOR_FRAIL;
    }
    value.max(0.0) as u16
}

// Strike Dummy: Strike-named Cards hit for 3 more
pub fn strike_dummy_bonus(name: CardName, id_relics: &[Option<usize>; RelicName::COUNT]) -> u16 {
    let strike = matches!(
        name,
        CardName::Strike | CardName::SneakyStrike | CardName::SwiftStrike
    );
    if strike && has_relic(id_relics, RelicName::StrikeDummy) {
        3
    } else {
        0
    }
}

// Wrist Blade: Attacks that cost 0 to play hit for 4 more
pub fn wrist_blade_bonus(
    card: &Entity,
    cost_effective: u16,
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> u16 {
    let applies = cost_effective == 0
        && card.card_kind == CardKind::Attack
        && !matches!(card.card_cost_kind, CardCostKind::XCost { .. })
        && has_relic(id_relics, RelicName::WristBlade);
    if applies { 4 } else { 0 }
}

// Relic tier from a roll against the caller's cuts
pub fn relic_tier_by_roll(roll: u8, th_common: u8, th_uncommon: u8) -> RelicTier {
    if roll < th_common {
        RelicTier::Common
    } else if roll < th_uncommon {
        RelicTier::Uncommon
    } else {
        RelicTier::Rare
    }
}

// Spawn gates for the Relics that have one, read against the owned set `id_relics`
pub fn relic_can_spawn(
    state: &GameState,
    id_relics: &[Option<usize>; RelicName::COUNT],
    name: RelicName,
) -> bool {
    let deck_has = |pred: fn(&Entity) -> bool| {
        state
            .id_card_deck
            .iter()
            .any(|&id| pred(&state.entities[id]))
    };
    match name {
        RelicName::BottledFlame => {
            deck_has(|c| c.card_kind == CardKind::Attack && c.card_rarity != CardRarity::Basic)
        }
        RelicName::BottledLightning => {
            deck_has(|c| c.card_kind == CardKind::Skill && c.card_rarity != CardRarity::Basic)
        }
        RelicName::BottledTornado => deck_has(|c| c.card_kind == CardKind::Power),
        RelicName::Girya | RelicName::PeacePipe | RelicName::Shovel => {
            [RelicName::Girya, RelicName::PeacePipe, RelicName::Shovel]
                .iter()
                .filter(|&&campfire| has_relic(id_relics, campfire))
                .count()
                < 2
        }
        RelicName::RingOfTheSerpent => has_relic(id_relics, RelicName::RingOfTheSnake),
        RelicName::Ectoplasm => state.act <= 1,
        RelicName::TheCourier
        | RelicName::MawBank
        | RelicName::OldCoin
        | RelicName::SmilingMask => state.room_kind_resolved != Some(RoomKind::Shop),
        _ => true,
    }
}

// Pop the run pool for `tier`, cascading Common -> Uncommon -> Rare -> Circlet and
// Shop -> Uncommon; a pick that fails its spawn gate is burned, the redraw from the back
pub fn draw_relic(
    state: &mut GameState,
    id_relics: &[Option<usize>; RelicName::COUNT],
    tier: RelicTier,
    from_back: bool,
) -> RelicName {
    let (pool, cascade) = match tier {
        RelicTier::Common => (&mut state.pool_relic_common, Some(RelicTier::Uncommon)),
        RelicTier::Uncommon => (&mut state.pool_relic_uncommon, Some(RelicTier::Rare)),
        RelicTier::Rare => (&mut state.pool_relic_rare, None),
        RelicTier::Shop => (&mut state.pool_relic_shop, Some(RelicTier::Uncommon)),
        RelicTier::Boss => (&mut state.pool_relic_boss, None),
        RelicTier::Starter | RelicTier::Special => {
            unreachable!("No random draws from tier {:?}", tier)
        }
    };
    if pool.is_empty() {
        return match cascade {
            Some(next) => draw_relic(state, id_relics, next, false),
            None => RelicName::Circlet,
        };
    }

    // The Boss pool pops the front from either entry point
    let name = if from_back && tier != RelicTier::Boss {
        pool.pop().unwrap()
    } else {
        pool.remove(0)
    };
    if relic_can_spawn(state, id_relics, name) && !has_relic(id_relics, name) {
        name
    } else {
        draw_relic(state, id_relics, tier, true)
    }
}

// Front draws until the pick is outside the excluded set; rejected picks stay consumed
pub fn draw_relic_excluding(
    state: &mut GameState,
    tier: RelicTier,
    exclusion: RelicExclusion,
) -> RelicName {
    let id_relics = state.id_relics;
    loop {
        let name = draw_relic(state, &id_relics, tier, false);
        let excluded = match exclusion {
            RelicExclusion::Unfiltered => false,
            RelicExclusion::Screenless => matches!(
                name,
                RelicName::BottledFlame
                    | RelicName::BottledLightning
                    | RelicName::BottledTornado
                    | RelicName::Whetstone
            ),
            RelicExclusion::NonCampfire => {
                matches!(
                    name,
                    RelicName::Girya | RelicName::PeacePipe | RelicName::Shovel
                )
            }
        };
        if !excluded {
            return name;
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
pub fn resolve_health_fraction(health_max: u16, amount: Amount) -> u16 {
    match amount {
        Amount::Absolute(a) => a,
        Amount::Relative {
            numerator,
            denominator,
        }
        | Amount::RelativeMinOne {
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
            match amount {
                Amount::RelativeMinOne { .. } => raw.max(1) as u16,
                _ => raw as u16,
            }
        }
        _ => unreachable!("health amounts resolve Absolute or Relative forms"),
    }
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

pub fn purge_price(purge_cost_run: u16, id_relics: &[Option<usize>; RelicName::COUNT]) -> u16 {
    if has_relic(id_relics, RelicName::SmilingMask) {
        return 50;
    }
    let base = purge_cost_run as u32;
    if has_relic(id_relics, RelicName::MembershipCard) {
        ((base + 1) / 2) as u16
    } else if has_relic(id_relics, RelicName::TheCourier) {
        ((base * 4 + 2) / 5) as u16
    } else {
        purge_cost_run
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

// Colorless offers never roll Common, so there is no Common colorless pool
const fn card_pool(rarity: CardRarity, colorless: bool) -> &'static [CardName] {
    match (rarity, colorless) {
        (CardRarity::Rare, false) => POOL_RARE_GREEN_CARD,
        (CardRarity::Uncommon, false) => POOL_UNCOMMON_GREEN_CARD,
        (_, false) => POOL_COMMON_GREEN_CARD,
        (CardRarity::Rare, true) => POOL_RARE_COLORLESS_CARD,
        (CardRarity::Uncommon, true) => POOL_UNCOMMON_COLORLESS_CARD,
        (_, true) => panic!("no Common colorless pool"),
    }
}

// Shop stock is not a reward, so it sits outside `RewardRollTrigger`
pub const SHOP_STOCK_POLICY: RollPolicy = RollPolicy {
    cuts: Some((SHOP_CARD_CUT_RARE, SHOP_CARD_CUT_UNCOMMON)), // Own bands
    colorless: false,
    read_pity: true,
    alternation: false,
    write_pity: false,         // Reads the pity without writing it
    dupe_rerolls_rarity: true, // make_card_colored's loop re-rolls rarity and Card together
    upgrade_roll: false,
    staged: true, // Kept, on offer as stock
};

// The one rarity roll, shared by Card rewards and Shop stock
pub fn roll_card_rarity(
    rng: &mut impl Rng,
    offset: i8,
    policy: &RollPolicy,
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> CardRarity {
    let (base_rare, base_uncommon) = policy.cuts.expect("an all-Rare roll never picks a rarity");
    let alternation = policy.alternation;

    // N'loths Gift: Triple the chance of receiving rare Cards
    let chance_rare = if alternation && has_relic(id_relics, RelicName::NlothsGift) {
        base_rare * 3
    } else {
        base_rare
    };

    // Roll; the pity offset only shifts it for consumers that read pity
    let offset = if policy.read_pity { offset as i32 } else { 0 };
    let roll = rng.random_range(0i32..=99) + offset;
    if roll < chance_rare {
        CardRarity::Rare
    } else if roll < chance_rare + (base_uncommon - base_rare) {
        CardRarity::Uncommon
    } else {
        CardRarity::Common
    }
}

// Cumulative Rare / Uncommon cuts; None skips the roll and every Card is Rare
pub struct RollPolicy {
    pub cuts: Option<(i32, i32)>,
    pub colorless: bool,
    pub read_pity: bool,
    pub alternation: bool,
    pub write_pity: bool,
    pub dupe_rerolls_rarity: bool,
    pub upgrade_roll: bool,
    pub staged: bool, // The rolled Cards are kept; a discarded roll only moves the pity offset
}

// The one place a consumer's roll rules live
pub const fn roll_policy(trigger: RewardRollTrigger) -> RollPolicy {
    const CUTS_MONSTER: Option<(i32, i32)> = Some((
        CARD_REWARD_ROLL_CHANCE_RARE,
        CARD_REWARD_ROLL_CHANCE_UNCOMMON,
    ));
    match trigger {
        RewardRollTrigger::CombatMonster | RewardRollTrigger::EventFight => RollPolicy {
            cuts: CUTS_MONSTER,
            colorless: false,
            read_pity: true,
            alternation: true,
            write_pity: true,
            dupe_rerolls_rarity: false,
            upgrade_roll: true,
            staged: true,
        },
        RewardRollTrigger::CombatElite => RollPolicy {
            cuts: Some((
                CARD_REWARD_ROLL_CHANCE_RARE_ELITE,
                CARD_REWARD_ROLL_CHANCE_UNCOMMON_ELITE,
            )),
            colorless: false,
            read_pity: true,
            alternation: true,
            write_pity: true,
            dupe_rerolls_rarity: false,
            upgrade_roll: true,
            staged: true,
        },
        RewardRollTrigger::CombatBoss => RollPolicy {
            cuts: None,
            colorless: false,
            read_pity: true,
            alternation: false,
            write_pity: true,
            dupe_rerolls_rarity: false,
            upgrade_roll: true,
            staged: true,
        },
        // Rest sites keep the default bands, but relics never widen them
        RewardRollTrigger::DreamCatcher => RollPolicy {
            cuts: CUTS_MONSTER,
            colorless: false,
            read_pity: true,
            alternation: false,
            write_pity: true,
            dupe_rerolls_rarity: false,
            upgrade_roll: true,
            staged: true,
        },
        // Bought in a Shop, so the offer rolls the shop bands and relics never widen them
        RewardRollTrigger::Orrery => RollPolicy {
            cuts: Some((SHOP_CARD_CUT_RARE, SHOP_CARD_CUT_UNCOMMON)),
            colorless: false,
            read_pity: true,
            alternation: false,
            write_pity: true,
            dupe_rerolls_rarity: false,
            upgrade_roll: true,
            staged: true,
        },
        RewardRollTrigger::Library => RollPolicy {
            cuts: CUTS_MONSTER,
            colorless: false,
            read_pity: true,
            alternation: true,
            write_pity: false,
            dupe_rerolls_rarity: true,
            upgrade_roll: false,
            staged: true,
        },
        // Neow: Uncommon or Common, never Rare unless the offer says so; no pity, never upgraded
        RewardRollTrigger::Neow => RollPolicy {
            cuts: Some((0, NEOW_UNCOMMON_CHANCE)),
            colorless: false,
            read_pity: false,
            alternation: false,
            write_pity: false,
            dupe_rerolls_rarity: false,
            upgrade_roll: false,
            staged: true,
        },
        RewardRollTrigger::NeowRare => RollPolicy {
            cuts: None,
            colorless: false,
            read_pity: false,
            alternation: false,
            write_pity: false,
            dupe_rerolls_rarity: false,
            upgrade_roll: false,
            staged: true,
        },
        // Neow's colorless offer is always Uncommon
        RewardRollTrigger::NeowColorless => RollPolicy {
            cuts: Some((0, 100)),
            colorless: true,
            read_pity: false,
            alternation: false,
            write_pity: false,
            dupe_rerolls_rarity: false,
            upgrade_roll: false,
            staged: true,
        },
        RewardRollTrigger::NeowColorlessRare => RollPolicy {
            cuts: None,
            colorless: true,
            read_pity: false,
            alternation: false,
            write_pity: false,
            dupe_rerolls_rarity: false,
            upgrade_roll: false,
            staged: true,
        },
        // The reward screens of Neow's Potions and Calling Bell roll a Card reward and discard it
        RewardRollTrigger::NeowPotions | RewardRollTrigger::CallingBell => RollPolicy {
            cuts: CUTS_MONSTER,
            colorless: false,
            read_pity: true,
            alternation: true,
            write_pity: true,
            dupe_rerolls_rarity: false,
            upgrade_roll: true,
            staged: false,
        },
        // Cauldron's screen does the same over a Shop, so its discarded roll uses the shop bands
        RewardRollTrigger::Cauldron => RollPolicy {
            cuts: Some((SHOP_CARD_CUT_RARE, SHOP_CARD_CUT_UNCOMMON)),
            colorless: false,
            read_pity: true,
            alternation: false,
            write_pity: true,
            dupe_rerolls_rarity: false,
            upgrade_roll: true,
            staged: false,
        },
        RewardRollTrigger::EventFightUnpaid
        | RewardRollTrigger::SmokeBomb
        | RewardRollTrigger::WomanInBlue
        | RewardRollTrigger::Lab
        | RewardRollTrigger::TinyHouse => panic!("no Card roll for this trigger"),
    }
}

// How a Potion reward rolls: a drifting drop chance or a sure thing, rarity-weighted or flat
pub struct PotionRollPolicy {
    pub drop_chance: bool,
    pub uniform: bool,
    pub staged: bool,
}

// The one place a Potion consumer's roll rules live
pub const fn potion_roll_policy(trigger: RewardRollTrigger) -> PotionRollPolicy {
    match trigger {
        // The end-of-combat drop: a drifting chance, rarity-weighted
        RewardRollTrigger::CombatMonster
        | RewardRollTrigger::CombatElite
        | RewardRollTrigger::CombatBoss
        | RewardRollTrigger::EventFight => PotionRollPolicy {
            drop_chance: true,
            uniform: false,
            staged: true,
        },
        // The chance still drifts, but nothing is kept
        RewardRollTrigger::SmokeBomb | RewardRollTrigger::EventFightUnpaid => PotionRollPolicy {
            drop_chance: true,
            uniform: false,
            staged: false,
        },
        // Granted outright, flat-uniform
        RewardRollTrigger::Cauldron
        | RewardRollTrigger::NeowPotions
        | RewardRollTrigger::WomanInBlue
        | RewardRollTrigger::Lab
        | RewardRollTrigger::TinyHouse => PotionRollPolicy {
            drop_chance: false,
            uniform: true,
            staged: true,
        },
        RewardRollTrigger::DreamCatcher
        | RewardRollTrigger::Orrery
        | RewardRollTrigger::Library
        | RewardRollTrigger::Neow
        | RewardRollTrigger::NeowRare
        | RewardRollTrigger::NeowColorless
        | RewardRollTrigger::NeowColorlessRare
        | RewardRollTrigger::CallingBell => panic!("no Potion roll for this trigger"),
    }
}

pub fn card_reward_upgrade_chance(act: u8, ascension_level: u8) -> f64 {
    if act < 2 {
        return 0.0;
    }
    if ascension_level >= ASCENSION_CARD_UPGRADE_CUT_LEVEL {
        CARD_REWARD_UPGRADE_CHANCE_ACT2_A12
    } else {
        CARD_REWARD_UPGRADE_CHANCE_ACT2
    }
}

pub fn roll_card_rewards(
    id_character: usize,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    out: &mut Vec<usize>,
    id_relics: &[Option<usize>; RelicName::COUNT],
    count: usize,
    trigger: RewardRollTrigger,
    act: u8,
    ascension_level: u8,
) {
    let policy = roll_policy(trigger);
    let mut character_reward_roll_offset = entities[id_character].character_reward_roll_offset;
    let mut card_names_rolled: [CardName; MAX_CARD_REWARD_ROLL] =
        [CardName::Strike; MAX_CARD_REWARD_ROLL];

    out.clear();
    for _ in 0..count {
        // Roll rarity
        let (mut pool, rarity) = if policy.cuts.is_none() {
            (
                card_pool(CardRarity::Rare, policy.colorless),
                CardRarity::Rare,
            )
        } else {
            let rarity = roll_card_rarity(rng, character_reward_roll_offset, &policy, id_relics);
            (card_pool(rarity, policy.colorless), rarity)
        };

        // Pity: reset offset on Rare hit; decrement on Common (toward more rares)
        if policy.write_pity {
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
            if policy.dupe_rerolls_rarity {
                pool = card_pool(
                    roll_card_rarity(rng, character_reward_roll_offset, &policy, id_relics),
                    policy.colorless,
                );
            }
            name = pool[rng.random_range(0..pool.len())];
        }
        card_names_rolled[out.len()] = name;

        // Push the rolled Card
        let card = get_card(
            name,
            // Eggs upgrade matching rewards at roll time, so the preview shows the truth
            egg_upgrades_kind(get_card_template(name, false).kind, id_relics),
        );
        let id_card = push_entity(entities, card);
        out.push(id_card);
    }

    // Pre-upgrade pass over the finished bundle
    let upgrade_chance = card_reward_upgrade_chance(act, ascension_level);
    if policy.upgrade_roll && upgrade_chance > 0.0 {
        for &id_card in out.iter() {
            let card = &entities[id_card];
            if card.card_rarity == CardRarity::Rare {
                continue;
            }
            if rng.random_bool(upgrade_chance) && card_is_upgradable(card) {
                let name = card.card_name;
                entities[id_card] = get_card(name, true);
            }
        }
    }

    entities[id_character].character_reward_roll_offset = character_reward_roll_offset;
}
