use rand::Rng;
use strum::EnumCount;

use crate::cards::get_random_cards;
use crate::consts::ASCENSION_SHOP_PRICE_BUMP_DENOM;
use crate::consts::ASCENSION_SHOP_PRICE_BUMP_LEVEL;
use crate::consts::ASCENSION_SHOP_PRICE_BUMP_NUMER;
use crate::consts::SHOP_CARD_TH_COMMON;
use crate::consts::SHOP_CARD_TH_UNCOMMON;
use crate::consts::SHOP_PRICE_CARD_COMMON;
use crate::consts::SHOP_PRICE_CARD_RARE;
use crate::consts::SHOP_PRICE_CARD_UNCOMMON;
use crate::consts::SHOP_PRICE_CARD_VARIANCE_MAX;
use crate::consts::SHOP_PRICE_CARD_VARIANCE_MIN;
use crate::consts::SHOP_PRICE_COLORLESS_DENOM;
use crate::consts::SHOP_PRICE_COLORLESS_NUMER;
use crate::consts::SHOP_PRICE_POTION_COMMON;
use crate::consts::SHOP_PRICE_POTION_RARE;
use crate::consts::SHOP_PRICE_POTION_UNCOMMON;
use crate::consts::SHOP_PRICE_RELIC_COMMON;
use crate::consts::SHOP_PRICE_RELIC_POTION_VARIANCE_MIN;
use crate::consts::SHOP_PRICE_RELIC_RARE;
use crate::consts::SHOP_PRICE_RELIC_SHOP;
use crate::consts::SHOP_PRICE_RELIC_UNCOMMON;
use crate::consts::SHOP_PURGE_COST_BASE;
use crate::consts::SHOP_RELIC_TH_COMMON;
use crate::consts::SHOP_RELIC_TH_UNCOMMON;
use crate::game::GameState;
use crate::game::clear_shop_state;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_SHOP_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::relics::get_relic;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::PotionRarity;
use crate::types::RelicName;
use crate::utils::pick_from_pool;
use crate::utils::push_entity;

pub fn process_effect_shop_build(state: &mut GameState) {
    clear_shop_state(state);

    // Colored: 2 Attack + 2 Skill + 1 Power
    push_card_colored(state, CardKind::Attack);
    push_card_colored(state, CardKind::Attack);
    push_card_colored(state, CardKind::Skill);
    push_card_colored(state, CardKind::Skill);
    push_card_colored(state, CardKind::Power);

    // Colorless: 1 Uncommon + 1 Rare
    push_card_colorless(state, CardRarity::Uncommon);
    push_card_colorless(state, CardRarity::Rare);

    // Relics: 2 random-tier, 1 shop-tier
    push_relic_random(state);
    push_relic_random(state);
    push_relic_shop(state);

    // Potions: 3 (rarity rolled by get_random_potion_name)
    push_potion(state);
    push_potion(state);
    push_potion(state);

    // A16+ price bumps
    if state.ascension >= ASCENSION_SHOP_PRICE_BUMP_LEVEL {
        for price in state.shop_card_prices.iter_mut() {
            *price = bump_price_a16(*price);
        }
        for price in state.shop_relic_prices.iter_mut() {
            *price = bump_price_a16(*price);
        }
        for price in state.shop_potion_prices.iter_mut() {
            *price = bump_price_a16(*price);
        }
    }
    state.shop_purge_cost = if state.ascension >= ASCENSION_SHOP_PRICE_BUMP_LEVEL {
        bump_price_a16(SHOP_PURGE_COST_BASE)
    } else {
        SHOP_PURGE_COST_BASE
    };

    // Sale tag: one random card 50% off
    if !state.shop_card_prices.is_empty() {
        let idx = state.rng.random_range(0..state.shop_card_prices.len());
        state.shop_card_prices[idx] /= 2;
    }
}

fn bump_price_a16(price: u16) -> u16 {
    (price as u32 * ASCENSION_SHOP_PRICE_BUMP_NUMER as u32 / ASCENSION_SHOP_PRICE_BUMP_DENOM as u32)
        as u16
}

fn roll_var_card(rng: &mut impl Rng) -> f32 {
    rng.random_range(SHOP_PRICE_CARD_VARIANCE_MIN..SHOP_PRICE_CARD_VARIANCE_MAX)
}

fn roll_var_relic_n_potion(rng: &mut impl Rng) -> f32 {
    rng.random_range(SHOP_PRICE_RELIC_POTION_VARIANCE_MIN..SHOP_PRICE_CARD_VARIANCE_MAX)
}

fn get_card_base_price(rarity: CardRarity) -> u16 {
    match rarity {
        CardRarity::Common => SHOP_PRICE_CARD_COMMON,
        CardRarity::Uncommon => SHOP_PRICE_CARD_UNCOMMON,
        CardRarity::Rare => SHOP_PRICE_CARD_RARE,
        _ => unreachable!("Shop only sells Common, Uncommon, or Rare cards"),
    }
}

// Card names already placed in this shop, so the shop's cards stay distinct
fn shop_placed_card_names(state: &GameState) -> Vec<CardName> {
    state
        .shop_id_cards
        .iter()
        .map(|&id| state.entities[id].card_name)
        .collect()
}

fn push_card_colored(state: &mut GameState, kind: CardKind) {
    // Roll rarity
    let mut rarity = roll_card_rarity(&mut state.rng);

    // No Common green Powers exist, so a Power slot can't be Common; bump it to Uncommon
    if kind == CardKind::Power && rarity == CardRarity::Common {
        rarity = CardRarity::Uncommon;
    }

    let placed = shop_placed_card_names(state);
    let entity = get_random_cards(
        CardColor::Green,
        Some(kind),
        Some(rarity),
        &placed,
        1,
        &mut state.rng,
    )
    .into_iter()
    .next()
    .unwrap_or_else(|| panic!("no shop card for kind {kind:?} rarity {rarity:?}"));

    let id = push_entity(&mut state.entities, entity);
    let price = (get_card_base_price(rarity) as f32 * roll_var_card(&mut state.rng)) as u16;
    state.shop_id_cards.push(id);
    state.shop_card_prices.push(price);
}

fn push_card_colorless(state: &mut GameState, rarity: CardRarity) {
    let placed = shop_placed_card_names(state);
    let Some(entity) = get_random_cards(
        CardColor::Colorless,
        None,
        Some(rarity),
        &placed,
        1,
        &mut state.rng,
    )
    .into_iter()
    .next() else {
        return;
    };
    let id = push_entity(&mut state.entities, entity);
    let base =
        get_card_base_price(rarity) * SHOP_PRICE_COLORLESS_NUMER / SHOP_PRICE_COLORLESS_DENOM;
    let price = (base as f32 * roll_var_card(&mut state.rng)) as u16;
    state.shop_id_cards.push(id);
    state.shop_card_prices.push(price);
}

// Owned relics plus those already placed in this shop, so the shop's relics stay distinct
fn shop_taken_relics(state: &GameState) -> [Option<usize>; RelicName::COUNT] {
    let mut taken = state.id_relics;
    for &id in &state.shop_id_relics {
        taken[state.entities[id].relic_name as usize] = Some(id);
    }
    taken
}

fn push_relic_random(state: &mut GameState) {
    let roll = state.rng.random_range(0..100) as u8;
    let (pool, base): (&[RelicName], u16) = if roll < SHOP_RELIC_TH_COMMON {
        (POOL_COMMON_RELIC, SHOP_PRICE_RELIC_COMMON)
    } else if roll < SHOP_RELIC_TH_UNCOMMON {
        (POOL_UNCOMMON_RELIC, SHOP_PRICE_RELIC_UNCOMMON)
    } else {
        (POOL_RARE_RELIC, SHOP_PRICE_RELIC_RARE)
    };
    let taken = shop_taken_relics(state);
    let Some(name) = pick_from_pool(pool, &taken, &mut state.rng) else {
        return;
    };
    push_relic_with_price(state, name, base);
}

fn push_relic_shop(state: &mut GameState) {
    let taken = shop_taken_relics(state);
    let Some(name) = pick_from_pool(POOL_SHOP_RELIC, &taken, &mut state.rng) else {
        return;
    };
    push_relic_with_price(state, name, SHOP_PRICE_RELIC_SHOP);
}

fn push_relic_with_price(state: &mut GameState, name: RelicName, base: u16) {
    let id = push_entity(&mut state.entities, get_relic(name));
    let price = (base as f32 * roll_var_relic_n_potion(&mut state.rng)) as u16;
    state.shop_id_relics.push(id);
    state.shop_relic_prices.push(price);
}

fn push_potion(state: &mut GameState) {
    let name = get_random_potion_name(&mut state.rng, false);
    let entity = get_potion(name);
    let base = match entity.potion_rarity {
        PotionRarity::Common => SHOP_PRICE_POTION_COMMON,
        PotionRarity::Uncommon => SHOP_PRICE_POTION_UNCOMMON,
        PotionRarity::Rare => SHOP_PRICE_POTION_RARE,
    };
    let id = push_entity(&mut state.entities, entity);
    let price = (base as f32 * roll_var_relic_n_potion(&mut state.rng)) as u16;
    state.shop_id_potions.push(id);
    state.shop_potion_prices.push(price);
}

fn roll_card_rarity(rng: &mut impl Rng) -> CardRarity {
    let roll = rng.random_range(0..100) as u8;
    if roll < SHOP_CARD_TH_COMMON {
        CardRarity::Common
    } else if roll < SHOP_CARD_TH_UNCOMMON {
        CardRarity::Uncommon
    } else {
        CardRarity::Rare
    }
}
