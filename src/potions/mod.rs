mod ancient;
mod attack;
mod blessing_of_the_forge;
mod block;
mod colorless;
mod cultist;
mod cunning;
mod dexterity;
mod distilled_chaos;
mod duplication;
mod energy;
mod entropic_brew;
mod essence_of_steel;
mod explosive;
mod fairy;
mod fear;
mod fire;
mod fruit_juice;
mod gamblers_brew;
mod ghost_in_a_jar;
mod liquid_bronze;
mod liquid_memories;
mod poison;
mod power;
mod regeneration;
mod skill;
mod smoke_bomb;
mod snecko_oil;
mod speed;
mod steroid;
mod strength;
mod swift;
mod weak;

use rand::Rng;
use strum::EnumCount;

use crate::consts::POTION_SLOTS_MAX;
use crate::consts::POTION_TH_COMMON;
use crate::consts::POTION_TH_UNCOMMON;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::ENTITY_ZERO;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::types::CostScope;
use crate::types::PotionName;
use crate::types::PotionRarity;

// Follows a CardDiscover roll; halts until the player picks from `id_discover`
pub const EFFECT_CARD_DISCOVER_PICK: Effect = Effect {
    kind: EffectKind::CardDiscoverPick {
        cost_zero: Some(CostScope::Turn),
    },
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Discover,
        filter: CandidateFilter::Any,
        selection_kind: SelectionKind::Input { count: 1 },
    },
};

// Totality relies on the len == COUNT and no-duplicate asserts below
const fn build_potion_by_name() -> [&'static Entity; PotionName::COUNT] {
    let mut buf = [ALL_POTIONS[0]; PotionName::COUNT];
    let mut i = 0;
    while i < ALL_POTIONS.len() {
        buf[ALL_POTIONS[i].potion_name as usize] = ALL_POTIONS[i];
        i += 1;
    }
    buf
}

static POTION_BY_NAME: [&'static Entity; PotionName::COUNT] = build_potion_by_name();

pub fn get_potion(name: PotionName) -> Entity {
    *POTION_BY_NAME[name as usize]
}

pub const ALL_POTIONS: &[&'static Entity] = &[
    &energy::POTION_ENERGY,
    &block::POTION_BLOCK,
    &strength::POTION_STRENGTH,
    &dexterity::POTION_DEXTERITY,
    &fire::POTION_FIRE,
    &explosive::POTION_EXPLOSIVE,
    &weak::POTION_WEAK,
    &fear::POTION_FEAR,
    &poison::POTION_POISON,
    &swift::POTION_SWIFT,
    &attack::POTION_ATTACK,
    &skill::POTION_SKILL,
    &power::POTION_POWER,
    &fruit_juice::POTION_FRUIT_JUICE,
    &ancient::POTION_ANCIENT,
    &liquid_bronze::POTION_LIQUID_BRONZE,
    &essence_of_steel::POTION_ESSENCE_OF_STEEL,
    &ghost_in_a_jar::POTION_GHOST_IN_A_JAR,
    &cultist::POTION_CULTIST,
    &cunning::POTION_CUNNING,
    &distilled_chaos::POTION_DISTILLED_CHAOS,
    &blessing_of_the_forge::POTION_BLESSING_OF_THE_FORGE,
    &entropic_brew::POTION_ENTROPIC_BREW,
    &regeneration::POTION_REGENERATION,
    &steroid::POTION_STEROID,
    &speed::POTION_SPEED,
    &duplication::POTION_DUPLICATION,
    &colorless::POTION_COLORLESS,
    &gamblers_brew::POTION_GAMBLERS_BREW,
    &liquid_memories::POTION_LIQUID_MEMORIES,
    &snecko_oil::POTION_SNECKO_OIL,
    &fairy::POTION_FAIRY,
    &smoke_bomb::POTION_SMOKE_BOMB,
];
// Assert all Potions are included without duplicates
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
    lhs as u8 == rhs as u8
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

// Get number of Potions per rarity-pool
const NUM_COMMON: usize = count_pool(PotionRarity::Common);
const NUM_UNCOMMON: usize = count_pool(PotionRarity::Uncommon);
const NUM_RARE: usize = count_pool(PotionRarity::Rare);

// Compute rarity-pools
const POOL_COMMON_POTION: &[PotionName] = &build_pool::<NUM_COMMON>(PotionRarity::Common);
const POOL_UNCOMMON_POTION: &[PotionName] = &build_pool::<NUM_UNCOMMON>(PotionRarity::Uncommon);
const POOL_RARE_POTION: &[PotionName] = &build_pool::<NUM_RARE>(PotionRarity::Rare);

// 65/25/10 tier roll; fall back to Common when the rolled tier is empty
pub fn get_random_potion_name(rng: &mut impl Rng, limited: bool) -> PotionName {
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
        return get_random_potion_name(rng, limited);
    }
    name
}

// Uniform over every Potion, ignoring rarity (Neow's Potions offer)
pub fn get_random_potion_name_uniform(rng: &mut impl Rng) -> PotionName {
    ALL_POTIONS[rng.random_range(0..ALL_POTIONS.len())].potion_name
}

pub fn find_free_slot(slots: &[Option<usize>; POTION_SLOTS_MAX], slots_max: u8) -> Option<usize> {
    let cap = (slots_max as usize).min(POTION_SLOTS_MAX);
    slots[..cap].iter().position(|s| s.is_none())
}

// Clear whichever belt slot holds id_potion; no-op if absent
pub fn remove_potion(id_potions: &mut [Option<usize>; POTION_SLOTS_MAX], id_potion_target: usize) {
    for id_potion in id_potions.iter_mut() {
        if *id_potion == Some(id_potion_target) {
            *id_potion = None;
            return;
        }
    }
}

pub const fn make_entity_potion(
    name: PotionName,
    rarity: PotionRarity,
    requires_target: bool,
    combat_only: bool,
    effects: &'static [Effect],
) -> Entity {
    Entity {
        kind: EntityKind::Potion,
        potion_name: name,
        potion_rarity: rarity,
        requires_target: requires_target,
        potion_combat_only: combat_only,
        potion_effects: effects,
        ..ENTITY_ZERO
    }
}
