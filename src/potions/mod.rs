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

use crate::consts::POTION_SLOTS_MAX;
use crate::consts::POTION_TH_COMMON;
use crate::consts::POTION_TH_UNCOMMON;
use crate::entity::Entity;
use crate::types::PotionName;

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

pub const POTION_POOL_COMMON: &[PotionName] = &[
    PotionName::EnergyPotion,
    PotionName::BlockPotion,
    PotionName::StrengthPotion,
    PotionName::DexterityPotion,
    PotionName::FirePotion,
    PotionName::ExplosivePotion,
    PotionName::WeakPotion,
    PotionName::FearPotion,
    PotionName::PoisonPotion,
    PotionName::SwiftPotion,
    PotionName::AttackPotion,
    PotionName::SkillPotion,
    PotionName::PowerPotion,
];

pub const POTION_POOL_UNCOMMON: &[PotionName] = &[];

pub const POTION_POOL_RARE: &[PotionName] = &[PotionName::FruitJuice];

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

// Append a fresh potion entity and write its id to the first free slot.
// On full slots, the entity still exists (orphaned) but is unreachable
pub fn grant_potion(
    entities: &mut Vec<Entity>,
    id_character: usize,
    name: PotionName,
) -> Option<usize> {
    let id_potion = entities.len();
    entities.push(get_potion(name));
    let character = &mut entities[id_character];
    if let Some(slot) = find_free_slot(&character.potion_slots, character.potion_slots_max) {
        character.potion_slots[slot] = Some(id_potion);
        Some(slot)
    } else {
        None
    }
}

pub fn take_potion(character: &mut Entity, idx_slot: usize) -> Option<usize> {
    character.potion_slots.get_mut(idx_slot).and_then(|s| s.take())
}
