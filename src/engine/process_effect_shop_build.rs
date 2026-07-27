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
use crate::consts::SHOP_SLOTS_CARD_TOTAL;
use crate::consts::SHOP_SLOTS_POTION;
use crate::consts::SHOP_SLOTS_RELIC;
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
use crate::utils::has_relic;
use crate::utils::pick_from_pool;
use crate::utils::push_entity;

pub fn process_effect_shop_build(state: &mut GameState) {
    let mut id_cards: Vec<usize> = Vec::with_capacity(SHOP_SLOTS_CARD_TOTAL);
    let mut id_relics: Vec<usize> = Vec::with_capacity(SHOP_SLOTS_RELIC);
    let mut id_potions: Vec<usize> = Vec::with_capacity(SHOP_SLOTS_POTION);

    // Colored: 2 Attack + 2 Skill + 1 Power
    push_card_colored(state, &mut id_cards, CardKind::Attack);
    push_card_colored(state, &mut id_cards, CardKind::Attack);
    push_card_colored(state, &mut id_cards, CardKind::Skill);
    push_card_colored(state, &mut id_cards, CardKind::Skill);
    push_card_colored(state, &mut id_cards, CardKind::Power);

    // Colorless: 1 Uncommon + 1 Rare
    push_card_colorless(state, &mut id_cards, CardRarity::Uncommon);
    push_card_colorless(state, &mut id_cards, CardRarity::Rare);

    // Relics: 2 random-tier, 1 shop-tier
    push_relic_random(state, &mut id_relics);
    push_relic_random(state, &mut id_relics);
    push_relic_shop(state, &mut id_relics);

    // Potions: 3 (rarity rolled by get_random_potion_name)
    push_potion(state, &mut id_potions);
    push_potion(state, &mut id_potions);
    push_potion(state, &mut id_potions);

    // Sale tag: one random colored card 50% off, before the A16 markup
    if !id_cards.is_empty() {
        let idx = state.rng.random_range(0..SHOP_SLOTS_CARD_COLORED);
        state.entities[id_cards[idx]].price /= 2;
    }

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

    state.mode = Mode::Shop {
        shop_id_cards: id_cards,
        shop_id_relics: id_relics,
        shop_id_potions: id_potions,
        shop_purge_cost: purge_cost,
    };
}

// The Courier ×0.8 then Membership Card ×0.5; sequential round-half-up (Java shop-init order)
pub(crate) fn apply_shop_discounts(
    price: u16,
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> u16 {
    let mut p = price as u32;
    if has_relic(id_relics, RelicName::TheCourier) {
        p = (p * 4 + 2) / 5;
    }
    if has_relic(id_relics, RelicName::MembershipCard) {
        p = (p + 1) / 2;
    }
    p as u16
}

// The Courier: restock a bought card slot in place; colored kind is kept with a rerolled
// rarity, colorless keeps its rarity. Discounted, but no A16 re-markup (Java parity)
pub(crate) fn restock_card(
    state: &mut GameState,
    id_cards: &mut Vec<usize>,
    idx: usize,
    color: CardColor,
    kind: CardKind,
    rarity: CardRarity,
) {
    if color == CardColor::Colorless {
        push_card_colorless(state, id_cards, rarity);
    } else {
        push_card_colored(state, id_cards, kind);
    }
    let id_new = id_cards.pop().expect("restock pushed a card");
    state.entities[id_new].price =
        apply_shop_discounts(state.entities[id_new].price, &state.id_relics);
    id_cards.insert(idx, id_new);
}

// The Courier: restock a bought relic slot; rerolls the tier, and the gold-economy
// relics never restock (Java blacklist)
pub(crate) fn restock_relic(state: &mut GameState, id_relics_vec: &mut Vec<usize>, idx: usize) {
    let roll = state.rng.random_range(0..100) as u8;
    let (pool, base_price): (&[RelicName], u16) = if roll < SHOP_RELIC_TH_COMMON {
        (POOL_COMMON_RELIC, SHOP_PRICE_RELIC_COMMON)
    } else if roll < SHOP_RELIC_TH_UNCOMMON {
        (POOL_UNCOMMON_RELIC, SHOP_PRICE_RELIC_UNCOMMON)
    } else {
        (POOL_RARE_RELIC, SHOP_PRICE_RELIC_RARE)
    };

    let mut taken = get_shop_taken_relic_names(state, id_relics_vec);
    for name in [
        RelicName::OldCoin,
        RelicName::SmilingMask,
        RelicName::MawBank,
    ] {
        taken[name as usize] = Some(usize::MAX);
    }
    let Some(name) = pick_from_pool(pool, &taken, &mut state.rng) else {
        return;
    };
    push_relic_with_price(state, id_relics_vec, name, base_price);
    let id_new = id_relics_vec.pop().expect("restock pushed a relic");
    state.entities[id_new].price =
        apply_shop_discounts(state.entities[id_new].price, &state.id_relics);
    id_relics_vec.insert(idx, id_new);
}

// The Courier: restock a bought potion slot
pub(crate) fn restock_potion(state: &mut GameState, id_potions: &mut Vec<usize>, idx: usize) {
    push_potion(state, id_potions);
    let id_new = id_potions.pop().expect("restock pushed a potion");
    state.entities[id_new].price =
        apply_shop_discounts(state.entities[id_new].price, &state.id_relics);
    id_potions.insert(idx, id_new);
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
fn get_shop_placed_card_names(state: &GameState, id_cards: &[usize]) -> Vec<CardName> {
    id_cards
        .iter()
        .map(|&id| state.entities[id].card_name)
        .collect()
}

// Sample one distinct shop card and push it with a variance-rolled price
fn push_card(
    state: &mut GameState,
    id_cards: &mut Vec<usize>,
    color: CardColor,
    kind: Option<CardKind>,
    rarity: CardRarity,
    base_price: u16,
) {
    // Sample card and its price
    let cards_placed = get_shop_placed_card_names(state, id_cards);
    let card = get_random_cards(color, kind, Some(rarity), &cards_placed, 1, &mut state.rng)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("no shop card for {color:?} {kind:?} rarity {rarity:?}"));
    let card_price = (base_price as f32 * roll_var_card(&mut state.rng)) as u16;

    // Push it
    let id_card = push_entity(&mut state.entities, card);
    state.entities[id_card].price = card_price;
    id_cards.push(id_card);
}

fn push_card_colored(state: &mut GameState, id_cards: &mut Vec<usize>, kind: CardKind) {
    let mut rarity = roll_card_rarity(&mut state.rng);

    // No Common green Powers exist, so a Power slot can't be Common; bump it to Uncommon
    if kind == CardKind::Power && rarity == CardRarity::Common {
        rarity = CardRarity::Uncommon;
    }

    push_card(
        state,
        id_cards,
        CardColor::Green,
        Some(kind),
        rarity,
        get_card_base_price(rarity),
    );
}

fn push_card_colorless(state: &mut GameState, id_cards: &mut Vec<usize>, rarity: CardRarity) {
    let base =
        get_card_base_price(rarity) * SHOP_PRICE_COLORLESS_NUMER / SHOP_PRICE_COLORLESS_DENOM;
    push_card(state, id_cards, CardColor::Colorless, None, rarity, base);
}

// Owned relics plus those already placed in this shop, so the shop's relics stay distinct
fn get_shop_taken_relic_names(
    state: &GameState,
    id_relics: &[usize],
) -> [Option<usize>; RelicName::COUNT] {
    let mut taken = state.id_relics;
    for &id in id_relics {
        taken[state.entities[id].relic_name as usize] = Some(id);
    }
    // The Courier never appears in shop stock (Java canSpawn blocks it inside shops)
    taken[RelicName::TheCourier as usize] = Some(usize::MAX);
    taken
}

fn push_relic_random(state: &mut GameState, id_relics: &mut Vec<usize>) {
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
    let taken = get_shop_taken_relic_names(state, id_relics);
    let Some(name) = pick_from_pool(pool, &taken, &mut state.rng) else {
        return;
    };
    push_relic_with_price(state, id_relics, name, base_price);
}

fn push_relic_shop(state: &mut GameState, id_relics: &mut Vec<usize>) {
    let taken = get_shop_taken_relic_names(state, id_relics);
    let Some(name) = pick_from_pool(POOL_SHOP_RELIC, &taken, &mut state.rng) else {
        return;
    };
    push_relic_with_price(state, id_relics, name, SHOP_PRICE_RELIC_SHOP);
}

fn push_relic_with_price(
    state: &mut GameState,
    id_relics: &mut Vec<usize>,
    name: RelicName,
    base_price: u16,
) {
    // Price it
    let id_relic = push_entity(&mut state.entities, get_relic(name));
    let relic_price = (base_price as f32 * roll_var_relic_n_potion(&mut state.rng)) as u16;

    // Push it
    state.entities[id_relic].price = relic_price;
    id_relics.push(id_relic);
}

fn push_potion(state: &mut GameState, id_potions: &mut Vec<usize>) {
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
    state.entities[id_potion].price = potion_price;
    id_potions.push(id_potion);
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
