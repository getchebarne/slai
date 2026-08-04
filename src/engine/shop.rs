// Shared shop-stock machinery: pricing, sampling, and The Courier's restocks.
// Not a processor; both the ShopBuild processor and the ShopBuy* processors use it

use rand::Rng;
use strum::EnumCount;

use crate::cards::get_random_cards;
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
use crate::consts::SHOP_PRICE_RELIC_POTION_VARIANCE_MAX;
use crate::consts::SHOP_PRICE_RELIC_POTION_VARIANCE_MIN;
use crate::consts::SHOP_PRICE_RELIC_RARE;
use crate::consts::SHOP_PRICE_RELIC_UNCOMMON;
use crate::consts::SHOP_RELIC_TH_COMMON;
use crate::consts::SHOP_RELIC_TH_UNCOMMON;
use crate::entity::Entity;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::relics::get_relic;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::PotionRarity;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::pick_relic_from_pool;
use crate::utils::push_entity;

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

// The Courier: restock a bought Relic slot; rerolls the tier
pub(super) fn restock_relic(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    id_relics: &[Option<usize>; RelicName::COUNT],
    id_relics_vec: &mut Vec<usize>,
    idx: usize,
) {
    let roll = rng.random_range(0..100) as u8;
    let (pool, base_price): (&[RelicName], u16) = if roll < SHOP_RELIC_TH_COMMON {
        (POOL_COMMON_RELIC, SHOP_PRICE_RELIC_COMMON)
    } else if roll < SHOP_RELIC_TH_UNCOMMON {
        (POOL_UNCOMMON_RELIC, SHOP_PRICE_RELIC_UNCOMMON)
    } else {
        (POOL_RARE_RELIC, SHOP_PRICE_RELIC_RARE)
    };

    // Get taken Relic IDs
    let mut id_taken = get_shop_taken_relic_names(id_relics, entities, id_relics_vec);

    // The gold-economy Relics never restock
    for name in [
        RelicName::OldCoin,
        RelicName::SmilingMask,
        RelicName::MawBank,
    ] {
        id_taken[name as usize] = Some(usize::MAX);
    }

    // Sample relic from pool
    let Some(name) = pick_relic_from_pool(pool, &id_taken, rng) else {
        return;
    };
    let id_relic_new = make_relic_with_price(entities, rng, name, base_price);

    // Apply discounts
    entities[id_relic_new].price = apply_shop_discounts(entities[id_relic_new].price, id_relics);

    // Insert it
    id_relics_vec.insert(idx, id_relic_new);
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
fn get_shop_placed_card_names(entities: &[Entity], id_cards: &[usize]) -> Vec<CardName> {
    id_cards.iter().map(|&id| entities[id].card_name).collect()
}

// Sample one distinct shop Card with a variance-rolled price; placement is the caller's
fn make_card(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    id_cards: &[usize],
    color: CardColor,
    kind: Option<CardKind>,
    rarity: CardRarity,
    base_price: u16,
) -> usize {
    // Sample Card and its price
    let cards_placed = get_shop_placed_card_names(entities, id_cards);
    let card = get_random_cards(color, kind, Some(rarity), &cards_placed, 1, rng)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("No shop Card for {color:?} {kind:?} rarity {rarity:?}"));
    let card_price = (base_price as f32 * roll_var_card(rng)) as u16;

    let id_card = push_entity(entities, card);
    entities[id_card].price = card_price;
    id_card
}

pub(super) fn make_card_colored(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    id_cards: &[usize],
    kind: CardKind,
) -> usize {
    let mut rarity = roll_card_rarity(rng);

    // No Common green Powers exist, so a Power slot can't be Common; bump it to Uncommon
    if kind == CardKind::Power && rarity == CardRarity::Common {
        rarity = CardRarity::Uncommon;
    }

    make_card(
        entities,
        rng,
        id_cards,
        CardColor::Green,
        Some(kind),
        rarity,
        get_card_base_price(rarity),
    )
}

pub(super) fn make_card_colorless(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    id_cards: &[usize],
    rarity: CardRarity,
) -> usize {
    let base =
        get_card_base_price(rarity) * SHOP_PRICE_COLORLESS_NUMER / SHOP_PRICE_COLORLESS_DENOM;
    make_card(
        entities,
        rng,
        id_cards,
        CardColor::Colorless,
        None,
        rarity,
        base,
    )
}

// Owned Relics plus those already placed in this shop, so the shop's Relics stay distinct
pub(super) fn get_shop_taken_relic_names(
    id_relics: &[Option<usize>; RelicName::COUNT],
    entities: &[Entity],
    id_relics_vec: &[usize],
) -> [Option<usize>; RelicName::COUNT] {
    let mut taken = *id_relics;
    for &id in id_relics_vec {
        taken[entities[id].relic_name as usize] = Some(id);
    }
    // The Courier never appears in shop stock (Java canSpawn blocks it inside shops)
    taken[RelicName::TheCourier as usize] = Some(usize::MAX);
    taken
}

pub(super) fn make_relic_with_price(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    name: RelicName,
    base_price: u16,
) -> usize {
    let id_relic = push_entity(entities, get_relic(name));
    let relic_price = (base_price as f32 * roll_var_relic_n_potion(rng)) as u16;
    entities[id_relic].price = relic_price;
    id_relic
}

pub(super) fn make_potion(entities: &mut Vec<Entity>, rng: &mut impl Rng) -> usize {
    // Sample Potion and its base price
    let name = get_random_potion_name(rng, false);
    let entity = get_potion(name);
    let base_price = match entity.potion_rarity {
        PotionRarity::Common => SHOP_PRICE_POTION_COMMON,
        PotionRarity::Uncommon => SHOP_PRICE_POTION_UNCOMMON,
        PotionRarity::Rare => SHOP_PRICE_POTION_RARE,
    };

    let id_potion = push_entity(entities, entity);
    let potion_price = (base_price as f32 * roll_var_relic_n_potion(rng)) as u16;
    entities[id_potion].price = potion_price;
    id_potion
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
