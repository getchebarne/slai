use rand::Rng;

use crate::consts::ASCENSION_SHOP_PRICE_BUMP_LEVEL;
use crate::consts::SHOP_PRICE_RELIC_SHOP;
use crate::consts::SHOP_SALE_DIVISOR;
use crate::consts::SHOP_SLOTS_CARD_COLORED;
use crate::consts::SHOP_SLOTS_POTION;
use crate::consts::bump_price_a16;
use crate::engine::shop::apply_shop_discounts;
use crate::engine::shop::make_card_colored;
use crate::engine::shop::make_card_colorless;
use crate::engine::shop::make_potion;
use crate::engine::shop::make_relic_with_price;
use crate::engine::shop::roll_shop_relic_tier;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardRarity;
use crate::types::RelicTier;
use crate::types::Shop;
use crate::types::shop_reset;
use crate::utils::draw_relic;
use crate::utils::purge_price;

pub fn process_effect_shop_build(state: &mut GameState) {
    // Stock builds straight into the context's retained buffers
    shop_reset(&mut state.shop);
    // Colored: 2 Attack + 2 Skill + 1 Power
    for kind in [
        CardKind::Attack,
        CardKind::Attack,
        CardKind::Skill,
        CardKind::Skill,
        CardKind::Power,
    ] {
        let card = make_card_colored(
            &mut state.entities,
            &mut state.rng,
            &state.shop.id_cards_price,
            kind,
            state.id_character,
            &state.id_relics,
        );
        state.shop.id_cards_price.push(card);
    }

    // Colorless: 1 Uncommon + 1 Rare
    for rarity in [CardRarity::Uncommon, CardRarity::Rare] {
        let card = make_card_colorless(
            &mut state.entities,
            &mut state.rng,
            &state.shop.id_cards_price,
            rarity,
        );
        state.shop.id_cards_price.push(card);
    }

    // Relics: 2 random-tier, 1 shop-tier, each drawn off the back of its run pool
    for _ in 0..2 {
        let (tier, base_price) = roll_shop_relic_tier(&mut state.rng);
        let name = draw_relic(state, tier, true);
        let offer = make_relic_with_price(&mut state.entities, &mut state.rng, name, base_price);
        state.shop.id_relics_price.push(offer);
    }
    let name = draw_relic(state, RelicTier::Shop, true);
    let offer = make_relic_with_price(
        &mut state.entities,
        &mut state.rng,
        name,
        SHOP_PRICE_RELIC_SHOP,
    );
    state.shop.id_relics_price.push(offer);

    // Potions: 3 (rarity rolled by get_random_potion_name)
    for _ in 0..SHOP_SLOTS_POTION {
        state
            .shop
            .id_potions_price
            .push(make_potion(&mut state.entities, &mut state.rng));
    }

    let Shop {
        id_cards_price,
        id_relics_price,
        id_potions_price,
        ..
    } = &mut state.shop;

    // Sale tag: one random colored Card 50% off, before the A16 markup
    let idx = state.rng.random_range(0..SHOP_SLOTS_CARD_COLORED);
    id_cards_price[idx].1 /= SHOP_SALE_DIVISOR;

    // A16+ price bumps; the purge cost is exempt
    if state.ascension >= ASCENSION_SHOP_PRICE_BUMP_LEVEL {
        for (_, price) in id_cards_price
            .iter_mut()
            .chain(&mut *id_relics_price)
            .chain(&mut *id_potions_price)
        {
            *price = bump_price_a16(*price);
        }
    }

    // The Courier / Membership Card: 20% / 50% off everything
    for (_, price) in id_cards_price
        .iter_mut()
        .chain(&mut *id_relics_price)
        .chain(&mut *id_potions_price)
    {
        *price = apply_shop_discounts(*price, &state.id_relics);
    }

    state.shop.purge_cost = purge_price(state.shop_purge_cost_run, &state.id_relics);
    state.shop.active = true;
}
