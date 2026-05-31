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
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::types::PotionName;
use crate::types::PotionRarity;
use crate::utils::push_entity;

// Follows a CardDiscover roll; halts until the player picks from `id_discover`
pub const EFFECT_CARD_DISCOVER_PICK: Effect = Effect {
    kind: EffectKind::CardDiscoverPick,
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Discover,
        selection_kind: SelectionKind::Input { count: 1 },
    },
};

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
// Assert all potions are included without duplicates
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

const fn potion_rarity_eq(lhs: PotionRarity, rhs: PotionRarity) -> bool {
    matches!(
        (lhs, rhs),
        (PotionRarity::Common, PotionRarity::Common)
            | (PotionRarity::Uncommon, PotionRarity::Uncommon)
            | (PotionRarity::Rare, PotionRarity::Rare)
    )
}

const fn count_pool(rarity: PotionRarity) -> usize {
    let mut count = 0;
    let mut idx = 0;
    while idx < ALL_POTIONS.len() {
        if potion_rarity_eq(ALL_POTIONS[idx].potion_rarity, rarity) {
            count += 1;
        }
        idx += 1;
    }
    count
}

const fn build_pool<const N: usize>(rarity: PotionRarity) -> [PotionName; N] {
    let mut buf = [PotionName::EnergyPotion; N];
    let mut idx_pool = 0;
    let mut idx_all = 0;
    while idx_all < ALL_POTIONS.len() {
        let potion = ALL_POTIONS[idx_all];
        if potion_rarity_eq(potion.potion_rarity, rarity) {
            buf[idx_pool] = potion.potion_name;
            idx_pool += 1;
        }
        idx_all += 1;
    }
    buf
}

// Get number of potions per rarity-pool
const NUM_COMMON: usize = count_pool(PotionRarity::Common);
const NUM_UNCOMMON: usize = count_pool(PotionRarity::Uncommon);
const NUM_RARE: usize = count_pool(PotionRarity::Rare);

// Compute rarity-pools
const POOL_COMMON_POTION: &[PotionName] = &build_pool::<NUM_COMMON>(PotionRarity::Common);
const POOL_UNCOMMON_POTION: &[PotionName] = &build_pool::<NUM_UNCOMMON>(PotionRarity::Uncommon);
const POOL_RARE_POTION: &[PotionName] = &build_pool::<NUM_RARE>(PotionRarity::Rare);

// 65/25/10 tier roll; fall back to Common when the rolled tier is empty
pub fn get_random_potion(rng: &mut impl Rng, limited: bool) -> PotionName {
    let roll = rng.random_range(0..100) as u8;
    let pool: &[PotionName] = if roll < POTION_TH_COMMON {
        POOL_COMMON_POTION
    } else if roll < POTION_TH_UNCOMMON {
        if POOL_UNCOMMON_POTION.is_empty() {
            POOL_COMMON_POTION
        } else {
            POOL_UNCOMMON_POTION
        }
    } else if POOL_RARE_POTION.is_empty() {
        POOL_COMMON_POTION
    } else {
        POOL_RARE_POTION
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
    let slot = match find_free_slot(
        &character.character_potion_slots,
        character.character_potion_slots_max,
    ) {
        Some(s) => s,
        None => return None,
    };
    let id_potion = push_entity(entities, get_potion(name));
    entities[id_character].character_potion_slots[slot] = Some(id_potion);
    Some(slot)
}

pub fn take_potion(character: &mut Entity, idx_slot: usize) -> Option<usize> {
    character
        .character_potion_slots
        .get_mut(idx_slot)
        .and_then(|s| s.take())
}
