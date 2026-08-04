use rand::Rng;

use crate::consts::ASCENSION_SHOP_PRICE_BUMP_DENOM;
use crate::consts::ASCENSION_SHOP_PRICE_BUMP_LEVEL;
use crate::consts::ASCENSION_SHOP_PRICE_BUMP_NUMER;
use crate::consts::SHOP_PRICE_RELIC_COMMON;
use crate::consts::SHOP_PRICE_RELIC_RARE;
use crate::consts::SHOP_PRICE_RELIC_SHOP;
use crate::consts::SHOP_PRICE_RELIC_UNCOMMON;
use crate::consts::SHOP_RELIC_TH_COMMON;
use crate::consts::SHOP_RELIC_TH_UNCOMMON;
use crate::consts::SHOP_SLOTS_CARD_COLORED;
use crate::consts::SHOP_SLOTS_CARD_TOTAL;
use crate::consts::SHOP_SLOTS_POTION;
use crate::consts::SHOP_SLOTS_RELIC;
use crate::engine::shop::apply_shop_discounts;
use crate::engine::shop::get_shop_taken_relic_names;
use crate::engine::shop::make_card_colored;
use crate::engine::shop::make_card_colorless;
use crate::engine::shop::make_potion;
use crate::engine::shop::make_relic_with_price;
use crate::game::GameState;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_SHOP_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::types::CardKind;
use crate::types::CardRarity;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::pick_relic_from_pool;

pub fn process_effect_shop_build(state: &mut GameState) {
    let mut id_cards: Vec<usize> = Vec::with_capacity(SHOP_SLOTS_CARD_TOTAL);
    let mut id_relics: Vec<usize> = Vec::with_capacity(SHOP_SLOTS_RELIC);
    let mut id_potions: Vec<usize> = Vec::with_capacity(SHOP_SLOTS_POTION);

    // Colored: 2 Attack + 2 Skill + 1 Power
    for kind in [
        CardKind::Attack,
        CardKind::Attack,
        CardKind::Skill,
        CardKind::Skill,
        CardKind::Power,
    ] {
        let id_card = make_card_colored(&mut state.entities, &mut state.rng, &id_cards, kind);
        id_cards.push(id_card);
    }

    // Colorless: 1 Uncommon + 1 Rare
    for rarity in [CardRarity::Uncommon, CardRarity::Rare] {
        let id_card = make_card_colorless(&mut state.entities, &mut state.rng, &id_cards, rarity);
        id_cards.push(id_card);
    }

    // Relics: 2 random-tier, 1 shop-tier. The tier roll stays per-slot for source parity
    for _ in 0..2 {
        let (pool, base_price) = roll_relic_tier(&mut state.rng);
        push_relic(state, &mut id_relics, pool, base_price);
    }
    push_relic(
        state,
        &mut id_relics,
        POOL_SHOP_RELIC,
        SHOP_PRICE_RELIC_SHOP,
    );

    // Potions: 3 (rarity rolled by get_random_potion_name)
    for _ in 0..SHOP_SLOTS_POTION {
        id_potions.push(make_potion(&mut state.entities, &mut state.rng));
    }

    // Sale tag: one random colored Card 50% off, before the A16 markup
    let idx = state.rng.random_range(0..SHOP_SLOTS_CARD_COLORED);
    state.entities[id_cards[idx]].price /= 2;

    // A16+ price bumps; the purge cost is exempt
    if state.ascension >= ASCENSION_SHOP_PRICE_BUMP_LEVEL {
        for &id in id_cards.iter().chain(&id_relics).chain(&id_potions) {
            state.entities[id].price = bump_price_a16(state.entities[id].price);
        }
    }

    // The Courier / Membership Card: 20% / 50% off everything
    for &id in id_cards.iter().chain(&id_relics).chain(&id_potions) {
        state.entities[id].price = apply_shop_discounts(state.entities[id].price, &state.id_relics);
    }

    // Smiling Mask: the removal service is always 50 gold
    let purge_cost = if has_relic(&state.id_relics, RelicName::SmilingMask) {
        50
    } else {
        apply_shop_discounts(state.shop_purge_cost_run, &state.id_relics)
    };

    state.mode_stack.push(Mode::Shop {
        shop_id_cards: id_cards,
        shop_id_relics: id_relics,
        shop_id_potions: id_potions,
        shop_purge_cost: purge_cost,
    });
}

fn bump_price_a16(price: u16) -> u16 {
    ((price as u32 * ASCENSION_SHOP_PRICE_BUMP_NUMER as u32
        + ASCENSION_SHOP_PRICE_BUMP_DENOM as u32 / 2)
        / ASCENSION_SHOP_PRICE_BUMP_DENOM as u32) as u16
}

fn roll_relic_tier(rng: &mut impl Rng) -> (&'static [RelicName], u16) {
    let roll = rng.random_range(0..100) as u8;
    if roll < SHOP_RELIC_TH_COMMON {
        (POOL_COMMON_RELIC, SHOP_PRICE_RELIC_COMMON)
    } else if roll < SHOP_RELIC_TH_UNCOMMON {
        (POOL_UNCOMMON_RELIC, SHOP_PRICE_RELIC_UNCOMMON)
    } else {
        (POOL_RARE_RELIC, SHOP_PRICE_RELIC_RARE)
    }
}

fn push_relic(
    state: &mut GameState,
    id_relics: &mut Vec<usize>,
    pool: &[RelicName],
    base_price: u16,
) {
    let id_taken = get_shop_taken_relic_names(&state.id_relics, &state.entities, id_relics);
    let Some(name) = pick_relic_from_pool(pool, &id_taken, &mut state.rng) else {
        return;
    };
    let id_relic = make_relic_with_price(&mut state.entities, &mut state.rng, name, base_price);
    id_relics.push(id_relic);
}
