mod attack_potion;
mod block_potion;
mod dexterity_potion;
mod energy_potion;
mod explosive_potion;
mod fear_potion;
mod fire_potion;
mod fruit_juice;
mod poison_potion;
mod power_potion;
mod skill_potion;
mod strength_potion;
mod swift_potion;
mod weak_potion;

use rand::Rng;
use strum::EnumCount;

use crate::consts::POTION_SLOTS_MAX;
use crate::consts::POTION_TH_COMMON;
use crate::consts::POTION_TH_UNCOMMON;
use crate::entity::Entity;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub fn get_potion(name: PotionName) -> Entity {
    match name {
        PotionName::EnergyPotion => energy_potion::ENERGY_POTION,
        PotionName::BlockPotion => block_potion::BLOCK_POTION,
        PotionName::StrengthPotion => strength_potion::STRENGTH_POTION,
        PotionName::DexterityPotion => dexterity_potion::DEXTERITY_POTION,
        PotionName::FirePotion => fire_potion::FIRE_POTION,
        PotionName::ExplosivePotion => explosive_potion::EXPLOSIVE_POTION,
        PotionName::WeakPotion => weak_potion::WEAK_POTION,
        PotionName::FearPotion => fear_potion::FEAR_POTION,
        PotionName::PoisonPotion => poison_potion::POISON_POTION,
        PotionName::SwiftPotion => swift_potion::SWIFT_POTION,
        PotionName::AttackPotion => attack_potion::ATTACK_POTION,
        PotionName::SkillPotion => skill_potion::SKILL_POTION,
        PotionName::PowerPotion => power_potion::POWER_POTION,
        PotionName::FruitJuice => fruit_juice::FRUIT_JUICE,
    }
}

pub const ALL_POTIONS: &[&'static Entity] = &[
    &energy_potion::ENERGY_POTION,
    &block_potion::BLOCK_POTION,
    &strength_potion::STRENGTH_POTION,
    &dexterity_potion::DEXTERITY_POTION,
    &fire_potion::FIRE_POTION,
    &explosive_potion::EXPLOSIVE_POTION,
    &weak_potion::WEAK_POTION,
    &fear_potion::FEAR_POTION,
    &poison_potion::POISON_POTION,
    &swift_potion::SWIFT_POTION,
    &attack_potion::ATTACK_POTION,
    &skill_potion::SKILL_POTION,
    &power_potion::POWER_POTION,
    &fruit_juice::FRUIT_JUICE,
];

const fn potion_rarity_eq(a: PotionRarity, b: PotionRarity) -> bool {
    matches!(
        (a, b),
        (PotionRarity::Common, PotionRarity::Common)
            | (PotionRarity::Uncommon, PotionRarity::Uncommon)
            | (PotionRarity::Rare, PotionRarity::Rare)
    )
}

const fn count_potion_pool(rarity: PotionRarity) -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < ALL_POTIONS.len() {
        if potion_rarity_eq(ALL_POTIONS[i].potion_rarity, rarity) {
            n += 1;
        }
        i += 1;
    }
    n
}

const fn build_potion_pool<const N: usize>(rarity: PotionRarity) -> [PotionName; N] {
    let mut buf = [PotionName::EnergyPotion; N];
    let mut idx = 0;
    let mut i = 0;
    while i < ALL_POTIONS.len() {
        let p = ALL_POTIONS[i];
        if potion_rarity_eq(p.potion_rarity, rarity) {
            buf[idx] = p.potion_name;
            idx += 1;
        }
        i += 1;
    }
    buf
}

const _: () = assert!(ALL_POTIONS.len() == PotionName::COUNT);
const _: () = {
    let mut seen = [false; PotionName::COUNT];
    let mut i = 0;
    while i < ALL_POTIONS.len() {
        let idx = ALL_POTIONS[i].potion_name as usize;
        assert!(!seen[idx], "ALL_POTIONS contains a duplicate PotionName");
        seen[idx] = true;
        i += 1;
    }
};

const COMMON_POTION_N: usize = count_potion_pool(PotionRarity::Common);
const UNCOMMON_POTION_N: usize = count_potion_pool(PotionRarity::Uncommon);
const RARE_POTION_N: usize = count_potion_pool(PotionRarity::Rare);

pub const POTION_POOL_COMMON: &[PotionName] =
    &build_potion_pool::<COMMON_POTION_N>(PotionRarity::Common);
pub const POTION_POOL_UNCOMMON: &[PotionName] =
    &build_potion_pool::<UNCOMMON_POTION_N>(PotionRarity::Uncommon);
pub const POTION_POOL_RARE: &[PotionName] =
    &build_potion_pool::<RARE_POTION_N>(PotionRarity::Rare);

// 65/25/10 tier roll; fall back to Common when the rolled tier is empty
// (slai-only fallback for sparse S3 pools, snaps back once Uncommon ships)
pub fn get_random_potion(rng: &mut impl Rng, limited: bool) -> PotionName {
    let roll = rng.random_range(0..100) as u8;
    let pool: &[PotionName] = if roll < POTION_TH_COMMON {
        POTION_POOL_COMMON
    } else if roll < POTION_TH_UNCOMMON {
        if POTION_POOL_UNCOMMON.is_empty() {
            POTION_POOL_COMMON
        } else {
            POTION_POOL_UNCOMMON
        }
    } else if POTION_POOL_RARE.is_empty() {
        POTION_POOL_COMMON
    } else {
        POTION_POOL_RARE
    };
    let name = pool[rng.random_range(0..pool.len())];
    if limited && name == PotionName::FruitJuice {
        return get_random_potion(rng, limited);
    }
    name
}

pub fn find_free_slot(slots: &[Option<usize>; POTION_SLOTS_MAX], slots_max: u8) -> Option<usize> {
    let cap = (slots_max as usize).min(POTION_SLOTS_MAX);
    slots[..cap].iter().position(|s| s.is_none())
}

// Returns the slot index on success, None when slots are full
pub fn grant_potion(
    entities: &mut Vec<Entity>,
    id_character: usize,
    name: PotionName,
) -> Option<usize> {
    let character = &entities[id_character];
    let slot = match find_free_slot(&character.potion_slots, character.potion_slots_max) {
        Some(s) => s,
        None => return None,
    };
    let id_potion = entities.len();
    entities.push(get_potion(name));
    entities[id_character].potion_slots[slot] = Some(id_potion);
    Some(slot)
}

pub fn take_potion(character: &mut Entity, idx_slot: usize) -> Option<usize> {
    character.potion_slots.get_mut(idx_slot).and_then(|s| s.take())
}
