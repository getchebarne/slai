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
use crate::consts::SHOP_RELIC_TH_COMMON;
use crate::consts::SHOP_RELIC_TH_UNCOMMON;
use crate::consts::SHOP_SLOTS_CARD_COLORED;
use crate::game::GameState;
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
use crate::types::Mode;
use crate::types::PotionRarity;
use crate::types::RelicName;
use crate::types::Shop;
use crate::utils::has_relic;
use crate::utils::pick_from_pool;
use crate::utils::push_entity;

pub fn process_effect_shop_build(state: &mut GameState) {
    let mut shop = Shop {
        id_cards: Vec::new(),
        id_relics: Vec::new(),
        id_potions: Vec::new(),
        card_prices: Vec::new(),
        relic_prices: Vec::new(),
        potion_prices: Vec::new(),
        purge_cost: 0,
    };

    // Colored: 2 Attack + 2 Skill + 1 Power
    push_card_colored(state, &mut shop, CardKind::Attack);
    push_card_colored(state, &mut shop, CardKind::Attack);
    push_card_colored(state, &mut shop, CardKind::Skill);
    push_card_colored(state, &mut shop, CardKind::Skill);
    push_card_colored(state, &mut shop, CardKind::Power);

    // Colorless: 1 Uncommon + 1 Rare
    push_card_colorless(state, &mut shop, CardRarity::Uncommon);
    push_card_colorless(state, &mut shop, CardRarity::Rare);

    // Relics: 2 random-tier, 1 shop-tier
    push_relic_random(state, &mut shop);
    push_relic_random(state, &mut shop);
    push_relic_shop(state, &mut shop);

    // Potions: 3 (rarity rolled by get_random_potion_name)
    push_potion(state, &mut shop);
    push_potion(state, &mut shop);
    push_potion(state, &mut shop);

    // Sale tag: one random colored card 50% off, before the A16 markup
    if !shop.card_prices.is_empty() {
        let idx = state.rng.random_range(0..SHOP_SLOTS_CARD_COLORED);
        shop.card_prices[idx] /= 2;
    }

    // A16+ price bumps; the purge cost is exempt
    if state.ascension >= ASCENSION_SHOP_PRICE_BUMP_LEVEL {
        for price in shop.card_prices.iter_mut() {
            *price = bump_price_a16(*price);
        }
        for price in shop.relic_prices.iter_mut() {
            *price = bump_price_a16(*price);
        }
        for price in shop.potion_prices.iter_mut() {
            *price = bump_price_a16(*price);
        }
    }

    // Smiling Mask: the removal service is always 50 gold
    shop.purge_cost = if has_relic(&state.id_relics, RelicName::SmilingMask) {
        50
    } else {
        state.shop_purge_cost_run
    };

    state.mode = Mode::Shop(shop);
}

fn bump_price_a16(price: u16) -> u16 {
    ((price as u32 * ASCENSION_SHOP_PRICE_BUMP_NUMER as u32
        + ASCENSION_SHOP_PRICE_BUMP_DENOM as u32 / 2)
        / ASCENSION_SHOP_PRICE_BUMP_DENOM as u32) as u16
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
fn get_shop_placed_card_names(state: &GameState, shop: &Shop) -> Vec<CardName> {
    shop.id_cards
        .iter()
        .map(|&id| state.entities[id].card_name)
        .collect()
}

// Sample one distinct shop card and push it with a variance-rolled price
fn push_card(
    state: &mut GameState,
    shop: &mut Shop,
    color: CardColor,
    kind: Option<CardKind>,
    rarity: CardRarity,
    base_price: u16,
) {
    // Sample card and its price
    let cards_placed = get_shop_placed_card_names(state, shop);
    let card = get_random_cards(color, kind, Some(rarity), &cards_placed, 1, &mut state.rng)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("no shop card for {color:?} {kind:?} rarity {rarity:?}"));
    let card_price = (base_price as f32 * roll_var_card(&mut state.rng)) as u16;

    // Push it
    let id_card = push_entity(&mut state.entities, card);
    shop.id_cards.push(id_card);
    shop.card_prices.push(card_price);
}

fn push_card_colored(state: &mut GameState, shop: &mut Shop, kind: CardKind) {
    let mut rarity = roll_card_rarity(&mut state.rng);

    // No Common green Powers exist, so a Power slot can't be Common; bump it to Uncommon
    if kind == CardKind::Power && rarity == CardRarity::Common {
        rarity = CardRarity::Uncommon;
    }

    push_card(
        state,
        shop,
        CardColor::Green,
        Some(kind),
        rarity,
        get_card_base_price(rarity),
    );
}

fn push_card_colorless(state: &mut GameState, shop: &mut Shop, rarity: CardRarity) {
    let base =
        get_card_base_price(rarity) * SHOP_PRICE_COLORLESS_NUMER / SHOP_PRICE_COLORLESS_DENOM;
    push_card(state, shop, CardColor::Colorless, None, rarity, base);
}

// Owned relics plus those already placed in this shop, so the shop's relics stay distinct
fn get_shop_taken_relic_names(state: &GameState, shop: &Shop) -> [Option<usize>; RelicName::COUNT] {
    let mut taken = state.id_relics;
    for &id in &shop.id_relics {
        taken[state.entities[id].relic_name as usize] = Some(id);
    }
    taken
}

fn push_relic_random(state: &mut GameState, shop: &mut Shop) {
    // Roll tier pool and its base price
    let roll = state.rng.random_range(0..100) as u8;
    let (pool, base_price): (&[RelicName], u16) = if roll < SHOP_RELIC_TH_COMMON {
        (POOL_COMMON_RELIC, SHOP_PRICE_RELIC_COMMON)
    } else if roll < SHOP_RELIC_TH_UNCOMMON {
        (POOL_UNCOMMON_RELIC, SHOP_PRICE_RELIC_UNCOMMON)
    } else {
        (POOL_RARE_RELIC, SHOP_PRICE_RELIC_RARE)
    };

    // Sample relic and push it
    let taken = get_shop_taken_relic_names(state, shop);
    let Some(name) = pick_from_pool(pool, &taken, &mut state.rng) else {
        return;
    };
    push_relic_with_price(state, shop, name, base_price);
}

fn push_relic_shop(state: &mut GameState, shop: &mut Shop) {
    let taken = get_shop_taken_relic_names(state, shop);
    let Some(name) = pick_from_pool(POOL_SHOP_RELIC, &taken, &mut state.rng) else {
        return;
    };
    push_relic_with_price(state, shop, name, SHOP_PRICE_RELIC_SHOP);
}

fn push_relic_with_price(state: &mut GameState, shop: &mut Shop, name: RelicName, base_price: u16) {
    // Price it
    let id_relic = push_entity(&mut state.entities, get_relic(name));
    let relic_price = (base_price as f32 * roll_var_relic_n_potion(&mut state.rng)) as u16;

    // Push it
    shop.id_relics.push(id_relic);
    shop.relic_prices.push(relic_price);
}

fn push_potion(state: &mut GameState, shop: &mut Shop) {
    // Sample potion and its base price
    let name = get_random_potion_name(&mut state.rng, false);
    let entity = get_potion(name);
    let base_price = match entity.potion_rarity {
        PotionRarity::Common => SHOP_PRICE_POTION_COMMON,
        PotionRarity::Uncommon => SHOP_PRICE_POTION_UNCOMMON,
        PotionRarity::Rare => SHOP_PRICE_POTION_RARE,
    };

    // Push it
    let id_potion = push_entity(&mut state.entities, entity);
    let potion_price = (base_price as f32 * roll_var_relic_n_potion(&mut state.rng)) as u16;
    shop.id_potions.push(id_potion);
    shop.potion_prices.push(potion_price);
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
