// Shared shop-stock machinery: pricing, sampling, and The Courier's restocks.
// Not a processor; both the ShopBuild processor and the ShopBuy* processors use it

use rand::Rng;
use strum::EnumCount;

use crate::cards::get_random_cards;
use crate::consts::SHOP_COLORLESS_RARE_CHANCE;
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
use crate::consts::SHOP_PRICE_RELIC_POTION_VARIANCE_MAX;
use crate::consts::SHOP_PRICE_RELIC_POTION_VARIANCE_MIN;
use crate::consts::SHOP_PRICE_RELIC_RARE;
use crate::consts::SHOP_PRICE_RELIC_UNCOMMON;
use crate::consts::SHOP_RELIC_TH_COMMON;
use crate::consts::SHOP_RELIC_TH_UNCOMMON;
use crate::entity::Entity;
use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::relics::get_relic;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::PotionRarity;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::utils::SHOP_STOCK_POLICY;
use crate::utils::draw_relic;
use crate::utils::has_relic;
use crate::utils::push_entity;
use crate::utils::roll_card_rarity;

// setPrice: the Courier restock keeps one float through variance and both discounts
pub(super) fn make_card_restock(
    state: &mut GameState,
    color: CardColor,
    kind: CardKind,
) -> (usize, u16) {
    let colorless = color == CardColor::Colorless;
    let rarity = if colorless {
        // colorlessRareChance: a fresh roll, never the bought Card's rarity
        if state.rng.random::<f32>() < SHOP_COLORLESS_RARE_CHANCE {
            CardRarity::Rare
        } else {
            CardRarity::Uncommon
        }
    } else {
        roll_card_rarity(&mut state.rng, 0, &SHOP_STOCK_POLICY, &state.id_relics)
    };
    let cards_placed: Vec<CardName> = state
        .shop
        .id_cards_price
        .iter()
        .map(|&(id, _)| state.entities[id].card_name)
        .collect();
    let card = if colorless {
        get_random_cards(
            CardColor::Colorless,
            None,
            Some(rarity),
            &cards_placed,
            false,
            1,
            &mut state.rng,
        )
    } else {
        get_random_cards(
            CardColor::Green,
            Some(kind),
            Some(rarity),
            &cards_placed,
            false,
            1,
            &mut state.rng,
        )
    };
    let Some(card) = card.into_iter().next() else {
        return (usize::MAX, 0);
    };
    let mut price = get_card_base_price(rarity) as f32 * roll_var_card(&mut state.rng);
    if colorless {
        price *= SHOP_PRICE_COLORLESS_NUMER as f32 / SHOP_PRICE_COLORLESS_DENOM as f32;
    }
    if has_relic(&state.id_relics, RelicName::TheCourier) {
        price *= 0.8;
    }
    if has_relic(&state.id_relics, RelicName::MembershipCard) {
        price *= 0.5;
    }
    let id_card = push_entity(&mut state.entities, card);
    (id_card, price as u16)
}

// The Courier x0.8 then Membership Card x0.5; sequential round-half-up (Java shop-init order)
pub(super) fn apply_shop_discounts(
    price: u16,
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> u16 {
    let mut price_snap = price as u32;
    if has_relic(id_relics, RelicName::TheCourier) {
        price_snap = (price_snap * 4 + 2) / 5;
    }
    if has_relic(id_relics, RelicName::MembershipCard) {
        price_snap = (price_snap + 1) / 2;
    }
    price_snap as u16
}

// rollRelicTier with the shop's cuts and base prices
pub(super) fn roll_shop_relic_tier(rng: &mut impl Rng) -> (RelicTier, u16) {
    let roll = rng.random_range(0..100) as u8;
    if roll < SHOP_RELIC_TH_COMMON {
        (RelicTier::Common, SHOP_PRICE_RELIC_COMMON)
    } else if roll < SHOP_RELIC_TH_UNCOMMON {
        (RelicTier::Uncommon, SHOP_PRICE_RELIC_UNCOMMON)
    } else {
        (RelicTier::Rare, SHOP_PRICE_RELIC_RARE)
    }
}

// The Courier: restock a bought Relic slot; rerolls the tier, draws off the back of the pool
pub(super) fn restock_relic(
    state: &mut GameState,
    id_relics_settled: &[Option<usize>; RelicName::COUNT],
    idx: usize,
) {
    let (tier, base_price) = roll_shop_relic_tier(&mut state.rng);
    let name = draw_relic(state, tier, true);
    let (id_relic_new, price) =
        make_relic_with_price(&mut state.entities, &mut state.rng, name, base_price);
    state.shop.id_relics_price.insert(
        idx,
        (id_relic_new, apply_shop_discounts(price, id_relics_settled)),
    );
}

fn roll_var_card(rng: &mut impl Rng) -> f32 {
    rng.random_range(SHOP_PRICE_CARD_VARIANCE_MIN..SHOP_PRICE_CARD_VARIANCE_MAX)
}

fn roll_var_relic_n_potion(rng: &mut impl Rng) -> f32 {
    rng.random_range(SHOP_PRICE_RELIC_POTION_VARIANCE_MIN..SHOP_PRICE_RELIC_POTION_VARIANCE_MAX)
}

fn get_card_base_price(rarity: CardRarity) -> u16 {
    match rarity {
        CardRarity::Common => SHOP_PRICE_CARD_COMMON,
        CardRarity::Uncommon => SHOP_PRICE_CARD_UNCOMMON,
        CardRarity::Rare => SHOP_PRICE_CARD_RARE,
        _ => unreachable!("Shop only sells Common, Uncommon, or Rare Cards"),
    }
}

// Card names already placed in this shop, so the shop's Cards stay distinct
fn get_shop_placed_card_names(entities: &[Entity], cards: &[(usize, u16)]) -> Vec<CardName> {
    cards
        .iter()
        .map(|&(id, _)| entities[id].card_name)
        .collect()
}

// Sample one distinct shop Card with a variance-rolled price; placement is the caller's
fn make_card(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    cards: &[(usize, u16)],
    color: CardColor,
    kind: Option<CardKind>,
    rarity: CardRarity,
    base_price: u16,
) -> (usize, u16) {
    // Sample Card and its price
    let cards_placed = get_shop_placed_card_names(entities, cards);
    let card = get_random_cards(color, kind, Some(rarity), &cards_placed, false, 1, rng)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("No shop Card for {color:?} {kind:?} rarity {rarity:?}"));
    let card_price = (base_price as f32 * roll_var_card(rng)) as u16;

    (push_entity(entities, card), card_price)
}

pub(super) fn make_card_colored(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    cards: &[(usize, u16)],
    kind: CardKind,
    id_character: usize,
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> (usize, u16) {
    let mut rarity = roll_card_rarity(
        rng,
        entities[id_character].character_reward_roll_offset,
        &SHOP_STOCK_POLICY,
        id_relics,
    );

    // No Common green Powers exist, so a Power slot can't be Common; bump it to Uncommon
    if kind == CardKind::Power && rarity == CardRarity::Common {
        rarity = CardRarity::Uncommon;
    }

    make_card(
        entities,
        rng,
        cards,
        CardColor::Green,
        Some(kind),
        rarity,
        get_card_base_price(rarity),
    )
}

pub(super) fn make_card_colorless(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    cards: &[(usize, u16)],
    rarity: CardRarity,
) -> (usize, u16) {
    let base =
        get_card_base_price(rarity) * SHOP_PRICE_COLORLESS_NUMER / SHOP_PRICE_COLORLESS_DENOM;
    make_card(
        entities,
        rng,
        cards,
        CardColor::Colorless,
        None,
        rarity,
        base,
    )
}

// Relics that are never sold in shops
pub(super) fn make_relic_with_price(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    name: RelicName,
    base_price: u16,
) -> (usize, u16) {
    let id_relic = push_entity(entities, get_relic(name));
    let relic_price = (base_price as f32 * roll_var_relic_n_potion(rng) + 0.5) as u16;
    (id_relic, relic_price)
}

pub(super) fn make_potion(entities: &mut Vec<Entity>, rng: &mut impl Rng) -> (usize, u16) {
    // Sample Potion and its base price
    let name = get_random_potion_name(rng, false);
    let entity = get_potion(name);
    let base_price = match entity.potion_rarity {
        PotionRarity::Common => SHOP_PRICE_POTION_COMMON,
        PotionRarity::Uncommon => SHOP_PRICE_POTION_UNCOMMON,
        PotionRarity::Rare => SHOP_PRICE_POTION_RARE,
    };

    let id_potion = push_entity(entities, entity);
    let potion_price = (base_price as f32 * roll_var_relic_n_potion(rng) + 0.5) as u16;
    (id_potion, potion_price)
}
