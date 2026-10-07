// Shared shop-stock machinery: pricing, sampling, and The Courier's restocks.
// Not a processor; both the ShopBuild processor and the ShopBuy* processors use it

use rand::Rng;
use strum::EnumCount;

use crate::cards::get_card;
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
use crate::consts::SHOP_PRICE_RELIC_SHOP;
use crate::consts::SHOP_PRICE_RELIC_SPECIAL;
use crate::consts::SHOP_PRICE_RELIC_UNCOMMON;
use crate::consts::SHOP_RELIC_TH_COMMON;
use crate::consts::SHOP_RELIC_TH_UNCOMMON;
use crate::entity::Entity;
use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::relics::egg_upgrades_kind;
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
use crate::utils::relic_tier_by_roll;
use crate::utils::roll_card_rarity;

// The Courier restock keeps one float through variance and both discounts
pub(super) fn make_card_restock(
    state: &mut GameState,
    color: CardColor,
    kind: CardKind,
) -> (usize, u16) {
    let colorless = color == CardColor::Colorless;
    let card = if colorless {
        // A fresh rarity roll, never the bought Card's rarity
        let rarity = if state.rng.random::<f32>() < SHOP_COLORLESS_RARE_CHANCE {
            CardRarity::Rare
        } else {
            CardRarity::Uncommon
        };
        sample_card(&mut state.rng, CardColor::Colorless, None, rarity)
    } else {
        let offset = state.entities[state.id_character].character_reward_roll_offset;
        sample_card_colored(&mut state.rng, offset, kind, &state.id_relics)
    };
    let card = if egg_upgrades_kind(card.card_kind, &state.id_relics) {
        get_card(card.card_name, true)
    } else {
        card
    };
    let mut price = get_card_base_price(card.card_rarity) as f32 * roll_var_card(&mut state.rng);
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

// The Courier x0.8 then Membership Card x0.5, each rounded half-up in turn
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

pub(super) fn roll_shop_relic_tier(rng: &mut impl Rng) -> RelicTier {
    let roll = rng.random_range(0..100) as u8;
    relic_tier_by_roll(roll, SHOP_RELIC_TH_COMMON, SHOP_RELIC_TH_UNCOMMON)
}

// The Courier: restock a bought Relic slot; rerolls the tier, draws off the back of the pool
pub(super) fn restock_relic(
    state: &mut GameState,
    id_relics_settled: &[Option<usize>; RelicName::COUNT],
    idx: usize,
) {
    let tier = roll_shop_relic_tier(&mut state.rng);
    let name = draw_relic(state, id_relics_settled, tier, true);
    let (id_relic_new, price) = make_relic_with_price(&mut state.entities, &mut state.rng, name);
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

fn sample_card(
    rng: &mut impl Rng,
    color: CardColor,
    kind: Option<CardKind>,
    rarity: CardRarity,
) -> Entity {
    get_random_cards(color, kind, Some(rarity), &[], false, 1, rng)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("No shop Card for {color:?} {kind:?} rarity {rarity:?}"))
}

// A green Card of `kind` at a fresh rarity roll; no Power is Common, so a Common roll
// for a Power takes an Uncommon one
fn sample_card_colored(
    rng: &mut impl Rng,
    offset: i8,
    kind: CardKind,
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> Entity {
    let mut rarity = roll_card_rarity(rng, offset, &SHOP_STOCK_POLICY, id_relics);
    if kind == CardKind::Power && rarity == CardRarity::Common {
        rarity = CardRarity::Uncommon;
    }
    sample_card(rng, CardColor::Green, Some(kind), rarity)
}

// Price a drawn shop Card with its variance roll; placement is the caller's
fn make_card(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    card: Entity,
    base_price: u16,
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> (usize, u16) {
    let card_price = (base_price as f32 * roll_var_card(rng)) as u16;

    // Eggs upgrade matching stock up front, so the shelf shows the Card as obtained
    let card = if egg_upgrades_kind(card.card_kind, id_relics) {
        get_card(card.card_name, true)
    } else {
        card
    };

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
    // A Card already on the shelf re-rolls rarity and Card together
    let cards_placed = get_shop_placed_card_names(entities, cards);
    let offset = entities[id_character].character_reward_roll_offset;
    let card = loop {
        let card = sample_card_colored(rng, offset, kind, id_relics);
        if !cards_placed.contains(&card.card_name) {
            break card;
        }
    };
    let base_price = get_card_base_price(card.card_rarity);
    make_card(entities, rng, card, base_price, id_relics)
}

pub(super) fn make_card_colorless(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    rarity: CardRarity,
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> (usize, u16) {
    let card = sample_card(rng, CardColor::Colorless, None, rarity);
    let base =
        get_card_base_price(rarity) * SHOP_PRICE_COLORLESS_NUMER / SHOP_PRICE_COLORLESS_DENOM;
    make_card(entities, rng, card, base, id_relics)
}

// A shelf Relic is priced by its own tier, whichever tier its draw cascaded from
pub(super) fn make_relic_with_price(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    name: RelicName,
) -> (usize, u16) {
    let relic = get_relic(name);
    let base_price = match relic.relic_tier {
        RelicTier::Common => SHOP_PRICE_RELIC_COMMON,
        RelicTier::Uncommon => SHOP_PRICE_RELIC_UNCOMMON,
        RelicTier::Rare => SHOP_PRICE_RELIC_RARE,
        RelicTier::Shop => SHOP_PRICE_RELIC_SHOP,
        RelicTier::Special => SHOP_PRICE_RELIC_SPECIAL,
        RelicTier::Starter | RelicTier::Boss => {
            unreachable!("Shops never stock {:?} Relics", relic.relic_tier)
        }
    };
    let id_relic = push_entity(entities, relic);
    let relic_price = (base_price as f32 * roll_var_relic_n_potion(rng) + 0.5) as u16;
    (id_relic, relic_price)
}

pub(super) fn make_potion(
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    potency_doubled: bool,
) -> (usize, u16) {
    // Sample Potion and its base price
    let name = get_random_potion_name(rng, false);
    let entity = get_potion(name, potency_doubled);
    let base_price = match entity.potion_rarity {
        PotionRarity::Common => SHOP_PRICE_POTION_COMMON,
        PotionRarity::Uncommon => SHOP_PRICE_POTION_UNCOMMON,
        PotionRarity::Rare => SHOP_PRICE_POTION_RARE,
    };

    let id_potion = push_entity(entities, entity);
    let potion_price = (base_price as f32 * roll_var_relic_n_potion(rng) + 0.5) as u16;
    (id_potion, potion_price)
}
