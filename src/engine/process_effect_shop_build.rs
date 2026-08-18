use rand::Rng;
use strum::EnumCount;

use crate::consts::ASCENSION_SHOP_PRICE_BUMP_LEVEL;
use crate::consts::SHOP_PRICE_RELIC_COMMON;
use crate::consts::SHOP_PRICE_RELIC_RARE;
use crate::consts::SHOP_PRICE_RELIC_SHOP;
use crate::consts::SHOP_PRICE_RELIC_UNCOMMON;
use crate::consts::SHOP_RELIC_TH_COMMON;
use crate::consts::SHOP_RELIC_TH_UNCOMMON;
use crate::consts::SHOP_SALE_DIVISOR;
use crate::consts::SHOP_SLOTS_CARD_COLORED;
use crate::consts::SHOP_SLOTS_POTION;
use crate::consts::bump_price_a16;
use crate::engine::shop::apply_shop_discounts;
use crate::engine::shop::get_shop_taken_relic_names;
use crate::engine::shop::make_card_colored;
use crate::engine::shop::make_card_colorless;
use crate::engine::shop::make_potion;
use crate::engine::shop::make_relic_with_price;
use crate::entity::Entity;
use crate::game::GameState;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_SHOP_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::types::CardKind;
use crate::types::CardRarity;
use crate::types::RelicName;
use crate::types::Shop;
use crate::types::shop_reset;
use crate::utils::has_relic;
use crate::utils::pick_relic_from_pool;

pub fn process_effect_shop_build(state: &mut GameState) {
    // Stock builds straight into the context's retained buffers
    shop_reset(&mut state.shop);
    let Shop {
        cards,
        relics,
        potions,
        ..
    } = &mut state.shop;

    // Colored: 2 Attack + 2 Skill + 1 Power
    for kind in [
        CardKind::Attack,
        CardKind::Attack,
        CardKind::Skill,
        CardKind::Skill,
        CardKind::Power,
    ] {
        let offer = make_card_colored(&mut state.entities, &mut state.rng, cards, kind);
        cards.push(offer);
    }

    // Colorless: 1 Uncommon + 1 Rare
    for rarity in [CardRarity::Uncommon, CardRarity::Rare] {
        let offer = make_card_colorless(&mut state.entities, &mut state.rng, cards, rarity);
        cards.push(offer);
    }

    // Relics: 2 random-tier, 1 shop-tier. The tier roll stays per-slot for source parity
    for _ in 0..2 {
        let (pool, base_price) = roll_relic_tier(&mut state.rng);
        push_relic(
            &mut state.entities,
            &mut state.rng,
            &state.id_relics,
            relics,
            pool,
            base_price,
        );
    }
    push_relic(
        &mut state.entities,
        &mut state.rng,
        &state.id_relics,
        relics,
        POOL_SHOP_RELIC,
        SHOP_PRICE_RELIC_SHOP,
    );

    // Potions: 3 (rarity rolled by get_random_potion_name)
    for _ in 0..SHOP_SLOTS_POTION {
        potions.push(make_potion(&mut state.entities, &mut state.rng));
    }

    // Sale tag: one random colored Card 50% off, before the A16 markup
    let idx = state.rng.random_range(0..SHOP_SLOTS_CARD_COLORED);
    cards[idx].1 /= SHOP_SALE_DIVISOR;

    // A16+ price bumps; the purge cost is exempt
    if state.ascension >= ASCENSION_SHOP_PRICE_BUMP_LEVEL {
        for (_, price) in cards.iter_mut().chain(&mut *relics).chain(&mut *potions) {
            *price = bump_price_a16(*price);
        }
    }

    // The Courier / Membership Card: 20% / 50% off everything
    for (_, price) in cards.iter_mut().chain(&mut *relics).chain(&mut *potions) {
        *price = apply_shop_discounts(*price, &state.id_relics);
    }

    // Smiling Mask: the removal service is always 50 gold
    let purge_cost = if has_relic(&state.id_relics, RelicName::SmilingMask) {
        50
    } else {
        apply_shop_discounts(state.shop_purge_cost_run, &state.id_relics)
    };

    state.shop.purge_cost = purge_cost;
    state.shop.active = true;
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
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    id_relics: &[Option<usize>; RelicName::COUNT],
    relics: &mut Vec<(usize, u16)>,
    pool: &[RelicName],
    base_price: u16,
) {
    let id_taken = get_shop_taken_relic_names(id_relics, entities, relics);
    let Some(name) = pick_relic_from_pool(pool, &id_taken, rng) else {
        return;
    };
    relics.push(make_relic_with_price(entities, rng, name, base_price));
}
