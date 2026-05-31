use rand::Rng;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_COLORLESS_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_COLORLESS_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::get_card;
use crate::consts::ASCENSION_SHOP_PRICE_BUMP_DENOM;
use crate::consts::ASCENSION_SHOP_PRICE_BUMP_LEVEL;
use crate::consts::ASCENSION_SHOP_PRICE_BUMP_NUMER;
use crate::consts::SHOP_CARD_TH_COMMON;
use crate::consts::SHOP_CARD_TH_UNCOMMON;
use crate::consts::SHOP_PRICE_CARD_COMMON;
use crate::consts::SHOP_PRICE_CARD_RARE;
use crate::consts::SHOP_PRICE_CARD_UNCOMMON;
use crate::consts::SHOP_PRICE_CARD_VARIANCE_HI;
use crate::consts::SHOP_PRICE_CARD_VARIANCE_LO;
use crate::consts::SHOP_PRICE_COLORLESS_DENOM;
use crate::consts::SHOP_PRICE_COLORLESS_NUMER;
use crate::consts::SHOP_PRICE_POTION_COMMON;
use crate::consts::SHOP_PRICE_POTION_RARE;
use crate::consts::SHOP_PRICE_POTION_UNCOMMON;
use crate::consts::SHOP_PRICE_RELIC_COMMON;
use crate::consts::SHOP_PRICE_RELIC_POTION_VARIANCE_HI;
use crate::consts::SHOP_PRICE_RELIC_POTION_VARIANCE_LO;
use crate::consts::SHOP_PRICE_RELIC_RARE;
use crate::consts::SHOP_PRICE_RELIC_SHOP;
use crate::consts::SHOP_PRICE_RELIC_UNCOMMON;
use crate::consts::SHOP_PURGE_COST_BASE;
use crate::consts::SHOP_RELIC_TH_COMMON;
use crate::consts::SHOP_RELIC_TH_UNCOMMON;
use crate::game::GameState;
use crate::game::clear_shop_state;
use crate::potions::get_potion;
use crate::potions::get_random_potion;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_SHOP_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::relics::get_relic;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::PotionRarity;
use crate::types::RelicName;
use crate::utils::pick_from_pool;
use crate::utils::push_entity;

pub fn process_effect_shop_build(state: &mut GameState) {
    clear_shop_state(state);
    let ascension = state.ascension;

    // Colored: 2 Attack + 2 Skill + 1 Power
    push_colored_card(state, CardKind::Attack);
    push_colored_card(state, CardKind::Attack);
    push_colored_card(state, CardKind::Skill);
    push_colored_card(state, CardKind::Skill);
    push_colored_card(state, CardKind::Power);

    // Colorless: 1 Uncommon + 1 Rare
    push_colorless_card(state, POOL_UNCOMMON_COLORLESS_CARD, CardRarity::Uncommon);
    push_colorless_card(state, POOL_RARE_COLORLESS_CARD, CardRarity::Rare);

    // Relics: 2 random-tier, 1 shop-tier
    push_random_tier_relic(state);
    push_random_tier_relic(state);
    push_shop_tier_relic(state);

    // Potions: 3 (rarity rolled by get_random_potion)
    push_potion(state);
    push_potion(state);
    push_potion(state);

    if ascension >= ASCENSION_SHOP_PRICE_BUMP_LEVEL {
        for p in state.shop_card_prices.iter_mut() {
            *p = bump_a16(*p);
        }
        for p in state.shop_relic_prices.iter_mut() {
            *p = bump_a16(*p);
        }
        for p in state.shop_potion_prices.iter_mut() {
            *p = bump_a16(*p);
        }
    }

    // Sale tag: one random card 50% off
    if !state.shop_card_prices.is_empty() {
        let idx = state.rng.random_range(0..state.shop_card_prices.len());
        state.shop_card_prices[idx] /= 2;
    }

    state.shop_purge_cost = if ascension >= ASCENSION_SHOP_PRICE_BUMP_LEVEL {
        bump_a16(SHOP_PURGE_COST_BASE)
    } else {
        SHOP_PURGE_COST_BASE
    };
}

fn bump_a16(price: u16) -> u16 {
    (price as u32 * ASCENSION_SHOP_PRICE_BUMP_NUMER as u32 / ASCENSION_SHOP_PRICE_BUMP_DENOM as u32)
        as u16
}

fn roll_card_variance(rng: &mut impl Rng) -> f32 {
    rng.random_range(SHOP_PRICE_CARD_VARIANCE_LO..SHOP_PRICE_CARD_VARIANCE_HI)
}

fn roll_relic_potion_variance(rng: &mut impl Rng) -> f32 {
    rng.random_range(SHOP_PRICE_RELIC_POTION_VARIANCE_LO..SHOP_PRICE_RELIC_POTION_VARIANCE_HI)
}

fn card_base_price(rarity: CardRarity) -> u16 {
    match rarity {
        CardRarity::Common => SHOP_PRICE_CARD_COMMON,
        CardRarity::Uncommon => SHOP_PRICE_CARD_UNCOMMON,
        CardRarity::Rare => SHOP_PRICE_CARD_RARE,
        // Shop never sells Basic/Special/Curse; treat as Common
        _ => SHOP_PRICE_CARD_COMMON,
    }
}

fn push_colored_card(state: &mut GameState, kind: CardKind) {
    let rarity = roll_colored_rarity(&mut state.rng);
    let pool: &[CardName] = match rarity {
        CardRarity::Common => POOL_COMMON_GREEN_CARD,
        CardRarity::Uncommon => POOL_UNCOMMON_GREEN_CARD,
        CardRarity::Rare => POOL_RARE_GREEN_CARD,
        _ => POOL_COMMON_GREEN_CARD,
    };
    let Some(name) = pick_card_of_kind(pool, kind, &mut state.rng) else {
        return;
    };
    let entity = get_card(name, false);
    let id = push_entity(&mut state.entities, entity);
    let price = (card_base_price(rarity) as f32 * roll_card_variance(&mut state.rng)) as u16;
    state.shop_id_cards.push(id);
    state.shop_card_prices.push(price);
}

fn push_colorless_card(state: &mut GameState, pool: &[CardName], rarity: CardRarity) {
    if pool.is_empty() {
        return;
    }
    let name = pool[state.rng.random_range(0..pool.len())];
    let entity = get_card(name, false);
    let id = push_entity(&mut state.entities, entity);
    let base = card_base_price(rarity) * SHOP_PRICE_COLORLESS_NUMER / SHOP_PRICE_COLORLESS_DENOM;
    let price = (base as f32 * roll_card_variance(&mut state.rng)) as u16;
    state.shop_id_cards.push(id);
    state.shop_card_prices.push(price);
}

fn push_random_tier_relic(state: &mut GameState) {
    let roll = state.rng.random_range(0..100) as u8;
    let (pool, base): (&[RelicName], u16) = if roll < SHOP_RELIC_TH_COMMON {
        (POOL_COMMON_RELIC, SHOP_PRICE_RELIC_COMMON)
    } else if roll < SHOP_RELIC_TH_UNCOMMON {
        (POOL_UNCOMMON_RELIC, SHOP_PRICE_RELIC_UNCOMMON)
    } else {
        (POOL_RARE_RELIC, SHOP_PRICE_RELIC_RARE)
    };
    let Some(name) = pick_from_pool(pool, &state.id_relics, &mut state.rng) else {
        return;
    };
    push_relic_with_price(state, name, base);
}

fn push_shop_tier_relic(state: &mut GameState) {
    let Some(name) = pick_from_pool(POOL_SHOP_RELIC, &state.id_relics, &mut state.rng) else {
        return;
    };
    push_relic_with_price(state, name, SHOP_PRICE_RELIC_SHOP);
}

fn push_relic_with_price(state: &mut GameState, name: RelicName, base: u16) {
    let id = push_entity(&mut state.entities, get_relic(name));
    let price = (base as f32 * roll_relic_potion_variance(&mut state.rng)) as u16;
    state.shop_id_relics.push(id);
    state.shop_relic_prices.push(price);
}

fn push_potion(state: &mut GameState) {
    let name = get_random_potion(&mut state.rng, false);
    let entity = get_potion(name);
    let base = match entity.potion_rarity {
        PotionRarity::Common => SHOP_PRICE_POTION_COMMON,
        PotionRarity::Uncommon => SHOP_PRICE_POTION_UNCOMMON,
        PotionRarity::Rare => SHOP_PRICE_POTION_RARE,
    };
    let id = push_entity(&mut state.entities, entity);
    let price = (base as f32 * roll_relic_potion_variance(&mut state.rng)) as u16;
    state.shop_id_potions.push(id);
    state.shop_potion_prices.push(price);
}

fn roll_colored_rarity(rng: &mut impl Rng) -> CardRarity {
    let roll = rng.random_range(0..100) as u8;
    if roll < SHOP_CARD_TH_COMMON {
        CardRarity::Common
    } else if roll < SHOP_CARD_TH_UNCOMMON {
        CardRarity::Uncommon
    } else {
        CardRarity::Rare
    }
}

fn pick_card_of_kind(pool: &[CardName], kind: CardKind, rng: &mut impl Rng) -> Option<CardName> {
    let mut buf = [CardName::Strike; 64];
    let mut n = 0;
    for &name in pool {
        if get_card(name, false).card_kind == kind {
            buf[n] = name;
            n += 1;
            if n == buf.len() {
                break;
            }
        }
    }
    if n == 0 {
        None
    } else {
        Some(buf[rng.random_range(0..n)])
    }
}
