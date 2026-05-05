use rand::Rng;
use strum::EnumCount;

use crate::effect::{Effect, EffectKind};
use crate::types::{EncounterPool, MonsterEncounter, MonsterName};
use crate::utils::shuffle;

pub const ALL_ENCOUNTERS: &[MonsterEncounter] = &[
    MonsterEncounter::Cultist,
    MonsterEncounter::JawWorm,
    MonsterEncounter::TwoLouse,
    MonsterEncounter::SmallSlimes,
    MonsterEncounter::BlueSlaver,
    MonsterEncounter::RedSlaver,
    MonsterEncounter::Looter,
    MonsterEncounter::TwoFungiBeasts,
    MonsterEncounter::ThreeLouse,
    MonsterEncounter::LargeSlime,
    MonsterEncounter::LotsOfSlimes,
    MonsterEncounter::GremlinGang,
    MonsterEncounter::ExordiumThugs,
    MonsterEncounter::ExordiumWildlife,
    MonsterEncounter::GremlinNob,
    MonsterEncounter::Lagavulin,
    MonsterEncounter::ThreeSentries,
    MonsterEncounter::TheGuardian,
    MonsterEncounter::Hexaghost,
    MonsterEncounter::SlimeBoss,
];

pub const fn encounter_pool(e: MonsterEncounter) -> EncounterPool {
    match e {
        MonsterEncounter::Cultist
        | MonsterEncounter::JawWorm
        | MonsterEncounter::TwoLouse
        | MonsterEncounter::SmallSlimes => EncounterPool::EasyAct1,
        MonsterEncounter::BlueSlaver
        | MonsterEncounter::RedSlaver
        | MonsterEncounter::Looter
        | MonsterEncounter::TwoFungiBeasts
        | MonsterEncounter::ThreeLouse
        | MonsterEncounter::LargeSlime
        | MonsterEncounter::LotsOfSlimes
        | MonsterEncounter::GremlinGang
        | MonsterEncounter::ExordiumThugs
        | MonsterEncounter::ExordiumWildlife => EncounterPool::HardAct1,
        MonsterEncounter::GremlinNob
        | MonsterEncounter::Lagavulin
        | MonsterEncounter::ThreeSentries => EncounterPool::EliteAct1,
        MonsterEncounter::TheGuardian
        | MonsterEncounter::Hexaghost
        | MonsterEncounter::SlimeBoss => EncounterPool::BossAct1,
    }
}

pub const fn encounter_weight(e: MonsterEncounter) -> f32 {
    match e {
        // Easy pool weights
        MonsterEncounter::Cultist => 2.0,
        MonsterEncounter::JawWorm => 2.0,
        MonsterEncounter::TwoLouse => 2.0,
        MonsterEncounter::SmallSlimes => 2.0,
        
        // Hard pool weights
        MonsterEncounter::BlueSlaver => 2.0,
        MonsterEncounter::GremlinGang => 1.0,
        MonsterEncounter::Looter => 2.0,
        MonsterEncounter::LargeSlime => 2.0,
        MonsterEncounter::LotsOfSlimes => 1.0,
        MonsterEncounter::ExordiumThugs => 1.5,
        MonsterEncounter::ExordiumWildlife => 1.5,
        MonsterEncounter::RedSlaver => 1.0,
        MonsterEncounter::ThreeLouse => 2.0,
        MonsterEncounter::TwoFungiBeasts => 2.0,
        
        // Elite pool weights
        MonsterEncounter::GremlinNob => 1.0,
        MonsterEncounter::Lagavulin => 1.0,
        MonsterEncounter::ThreeSentries => 1.0,
        
        // Boss pool: not weighted
        MonsterEncounter::TheGuardian => 1.0,
        MonsterEncounter::Hexaghost => 1.0,
        MonsterEncounter::SlimeBoss => 1.0,
    }
}


const fn pool_eq(a: EncounterPool, b: EncounterPool) -> bool {
    matches!(
        (a, b),
        (EncounterPool::EasyAct1, EncounterPool::EasyAct1)
            | (EncounterPool::HardAct1, EncounterPool::HardAct1)
            | (EncounterPool::EliteAct1, EncounterPool::EliteAct1)
            | (EncounterPool::BossAct1, EncounterPool::BossAct1)
    )
}

const fn count_pool(pool: EncounterPool) -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < ALL_ENCOUNTERS.len() {
        if pool_eq(encounter_pool(ALL_ENCOUNTERS[i]), pool) {
            n += 1;
        }
        i += 1;
    }
    n
}

const fn build_pool<const N: usize>(pool: EncounterPool) -> [MonsterEncounter; N] {
    let mut buf = [MonsterEncounter::Cultist; N];
    let mut idx = 0;
    let mut i = 0;
    while i < ALL_ENCOUNTERS.len() {
        if pool_eq(encounter_pool(ALL_ENCOUNTERS[i]), pool) {
            buf[idx] = ALL_ENCOUNTERS[i];
            idx += 1;
        }
        i += 1;
    }
    buf
}

const _: () = assert!(ALL_ENCOUNTERS.len() == MonsterEncounter::COUNT);
const _: () = {
    let mut seen = [false; MonsterEncounter::COUNT];
    let mut i = 0;
    while i < ALL_ENCOUNTERS.len() {
        let idx = ALL_ENCOUNTERS[i] as usize;
        assert!(!seen[idx], "ALL_ENCOUNTERS contains a duplicate MonsterEncounter");
        seen[idx] = true;
        i += 1;
    }
};

const EASY_N: usize = count_pool(EncounterPool::EasyAct1);
const HARD_N: usize = count_pool(EncounterPool::HardAct1);
const ELITE_N: usize = count_pool(EncounterPool::EliteAct1);
const BOSS_N: usize = count_pool(EncounterPool::BossAct1);

const EASY_ARR: [MonsterEncounter; EASY_N] = build_pool(EncounterPool::EasyAct1);
const HARD_ARR: [MonsterEncounter; HARD_N] = build_pool(EncounterPool::HardAct1);
const ELITE_ARR: [MonsterEncounter; ELITE_N] = build_pool(EncounterPool::EliteAct1);
const BOSS_ARR: [MonsterEncounter; BOSS_N] = build_pool(EncounterPool::BossAct1);

pub const POOL_EASY_ACT1: &[MonsterEncounter] = &EASY_ARR;
pub const POOL_HARD_ACT1: &[MonsterEncounter] = &HARD_ARR;
pub const POOL_ELITE_ACT1: &[MonsterEncounter] = &ELITE_ARR;
pub const POOL_BOSS_ACT1: &[MonsterEncounter] = &BOSS_ARR;


// Sort ascending by weight (stable for ties), normalize to sum 1.0.
// Sorting is observable via roll's cumulative-walk short-circuit, so it
// must match Java's `Collections.sort` stable order.
fn normalize_weights(pool: &[MonsterEncounter]) -> Vec<(MonsterEncounter, f32)> {
    let mut table: Vec<(MonsterEncounter, f32)> =
        pool.iter().map(|&e| (e, encounter_weight(e))).collect();
    table.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let total: f32 = table.iter().map(|(_, w)| *w).sum();
    for (_, w) in &mut table {
        *w /= total;
    }
    table
}

fn roll(table: &[(MonsterEncounter, f32)], r: f32) -> MonsterEncounter {
    let mut cum = 0.0;
    for &(e, w) in table {
        cum += w;
        if r < cum {
            return e;
        }
    }
    // Float rounding can leave the final cumulative just below 1.0; return last
    table.last().unwrap().0
}


fn populate_monster_list(
    list: &mut Vec<MonsterEncounter>,
    table: &[(MonsterEncounter, f32)],
    n: usize,
    elites: bool,
    rng: &mut impl Rng,
) {
    let mut count = 0;
    while count < n {
        if list.is_empty() {
            list.push(roll(table, rng.random_range(0.0..1.0)));
            count += 1;
            continue;
        }
        let candidate = roll(table, rng.random_range(0.0..1.0));
        if candidate == *list.last().unwrap() {
            continue;
        }
        if !elites && list.len() >= 2 && candidate == list[list.len() - 2] {
            continue;
        }
        list.push(candidate);
        count += 1;
    }
}

fn populate_first_strong_enemy(
    list: &mut Vec<MonsterEncounter>,
    table: &[(MonsterEncounter, f32)],
    exclusions: &[MonsterEncounter],
    rng: &mut impl Rng,
) {
    loop {
        let m = roll(table, rng.random_range(0.0..1.0));
        if !exclusions.contains(&m) {
            list.push(m);
            return;
        }
    }
}

// Keyed on the last weak entry (the 3rd weak fight).
// In Exordium only the variants below can be a "last weak"; other Java
// cases are dead in this dungeon
fn act1_exclusions(last_weak: MonsterEncounter) -> &'static [MonsterEncounter] {
    match last_weak {
        MonsterEncounter::TwoLouse => &[MonsterEncounter::ThreeLouse],
        MonsterEncounter::SmallSlimes => {
            &[MonsterEncounter::LargeSlime, MonsterEncounter::LotsOfSlimes]
        }
        _ => &[],
    }
}


pub fn generate_act1_monsters(
    monster_list: &mut Vec<MonsterEncounter>,
    elite_list: &mut Vec<MonsterEncounter>,
    rng: &mut impl Rng,
) {
    let easy = normalize_weights(POOL_EASY_ACT1);
    let hard = normalize_weights(POOL_HARD_ACT1);
    let elite = normalize_weights(POOL_ELITE_ACT1);

    populate_monster_list(monster_list, &easy, 3, false, rng);
    let last_weak = *monster_list.last().unwrap();
    let exclusions = act1_exclusions(last_weak);
    populate_first_strong_enemy(monster_list, &hard, exclusions, rng);
    populate_monster_list(monster_list, &hard, 12, false, rng);
    populate_monster_list(elite_list, &elite, 10, true, rng);
}

pub fn generate_act1_strong_only(list: &mut Vec<MonsterEncounter>, rng: &mut impl Rng) {
    let hard = normalize_weights(POOL_HARD_ACT1);
    populate_monster_list(list, &hard, 12, false, rng);
}

pub fn generate_act1_elites(list: &mut Vec<MonsterEncounter>, rng: &mut impl Rng) {
    let elite = normalize_weights(POOL_ELITE_ACT1);
    populate_monster_list(list, &elite, 10, true, rng);
}


pub fn initialize_act1_boss(list: &mut Vec<MonsterEncounter>, rng: &mut impl Rng) {
    list.clear();
    list.push(MonsterEncounter::TheGuardian);
    list.push(MonsterEncounter::Hexaghost);
    list.push(MonsterEncounter::SlimeBoss);
    shuffle(list.as_mut_slice(), rng);
}


fn pick_louse(rng: &mut impl Rng) -> MonsterName {
    if rng.random_bool(0.5) {
        MonsterName::LouseNormal
    } else {
        MonsterName::LouseDefensive
    }
}

fn pick_slaver(rng: &mut impl Rng) -> MonsterName {
    if rng.random_bool(0.5) {
        MonsterName::SlaverRed
    } else {
        MonsterName::SlaverBlue
    }
}

fn pick_weak_wildlife(rng: &mut impl Rng) -> MonsterName {
    match rng.random_range(0..3) {
        0 => pick_louse(rng),
        1 => MonsterName::SlimeSpikeMedium,
        2 => MonsterName::SlimeAcidMedium,
        _ => unreachable!(),
    }
}

fn pick_strong_wildlife(rng: &mut impl Rng) -> MonsterName {
    if rng.random_bool(0.5) {
        MonsterName::FungiBeast
    } else {
        MonsterName::JawWorm
    }
}

fn pick_strong_humanoid(rng: &mut impl Rng) -> MonsterName {
    match rng.random_range(0..3) {
        0 => MonsterName::Cultist,
        1 => pick_slaver(rng),
        2 => MonsterName::Looter,
        _ => unreachable!(),
    }
}


#[inline]
fn push_spawn(effects: &mut Vec<Effect>, name: MonsterName) {
    effects.push(Effect::direct(EffectKind::MonsterSpawn { name }, None, None));
}

pub fn spawn_encounter(
    encounter: MonsterEncounter,
    rng: &mut impl Rng,
    effects: &mut Vec<Effect>,
) {
    match encounter {
        MonsterEncounter::Cultist => push_spawn(effects, MonsterName::Cultist),
        MonsterEncounter::JawWorm => push_spawn(effects, MonsterName::JawWorm),
        MonsterEncounter::TwoLouse => {
            for _ in 0..2 {
                push_spawn(effects, pick_louse(rng));
            }
        }
        MonsterEncounter::SmallSlimes => {
            let (small, medium) = if rng.random_bool(0.5) {
                (MonsterName::SlimeSpikeSmall, MonsterName::SlimeAcidMedium)
            } else {
                (MonsterName::SlimeAcidSmall, MonsterName::SlimeSpikeMedium)
            };
            push_spawn(effects, small);
            push_spawn(effects, medium);
        }
        MonsterEncounter::BlueSlaver => push_spawn(effects, MonsterName::SlaverBlue),
        MonsterEncounter::RedSlaver => push_spawn(effects, MonsterName::SlaverRed),
        MonsterEncounter::Looter => push_spawn(effects, MonsterName::Looter),
        MonsterEncounter::TwoFungiBeasts => {
            push_spawn(effects, MonsterName::FungiBeast);
            push_spawn(effects, MonsterName::FungiBeast);
        }
        MonsterEncounter::ThreeLouse => {
            for _ in 0..3 {
                push_spawn(effects, pick_louse(rng));
            }
        }
        MonsterEncounter::LargeSlime => {
            let name = if rng.random_bool(0.5) {
                MonsterName::SlimeAcidLarge
            } else {
                MonsterName::SlimeSpikeLarge
            };
            push_spawn(effects, name);
        }
        MonsterEncounter::LotsOfSlimes => {
            let mut pool = [
                MonsterName::SlimeSpikeSmall,
                MonsterName::SlimeSpikeSmall,
                MonsterName::SlimeSpikeSmall,
                MonsterName::SlimeAcidSmall,
                MonsterName::SlimeAcidSmall,
            ];
            shuffle(&mut pool, rng);
            for &name in &pool {
                push_spawn(effects, name);
            }
        }
        MonsterEncounter::GremlinGang => {
            let mut pool = [
                MonsterName::GremlinWarrior,
                MonsterName::GremlinWarrior,
                MonsterName::GremlinThief,
                MonsterName::GremlinThief,
                MonsterName::GremlinFat,
                MonsterName::GremlinFat,
                MonsterName::GremlinTsundere,
                MonsterName::GremlinWizard,
            ];
            shuffle(&mut pool, rng);
            for &name in &pool[..4] {
                push_spawn(effects, name);
            }
        }
        MonsterEncounter::ExordiumThugs => {
            push_spawn(effects, pick_weak_wildlife(rng));
            push_spawn(effects, pick_strong_humanoid(rng));
        }
        MonsterEncounter::ExordiumWildlife => {
            push_spawn(effects, pick_strong_wildlife(rng));
            push_spawn(effects, pick_weak_wildlife(rng));
        }
        MonsterEncounter::GremlinNob => push_spawn(effects, MonsterName::GremlinNob),
        MonsterEncounter::Lagavulin => push_spawn(effects, MonsterName::Lagavulin),
        MonsterEncounter::ThreeSentries => {
            for _ in 0..3 {
                push_spawn(effects, MonsterName::Sentry);
            }
        }
        MonsterEncounter::TheGuardian => push_spawn(effects, MonsterName::TheGuardian),
        MonsterEncounter::Hexaghost => push_spawn(effects, MonsterName::Hexaghost),
        MonsterEncounter::SlimeBoss => push_spawn(effects, MonsterName::SlimeBoss),
    }
}
