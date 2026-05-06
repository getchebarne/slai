use rand::Rng;
use strum::EnumCount;

use crate::types::{EncounterPool, MonsterEncounter};

pub const ALL_ENCOUNTERS: &[MonsterEncounter] = &[
    MonsterEncounter::BlueSlaver,
    MonsterEncounter::Cultist,
    MonsterEncounter::ExordiumThugs,
    MonsterEncounter::ExordiumWildlife,
    MonsterEncounter::GremlinGang,
    MonsterEncounter::GremlinNob,
    MonsterEncounter::Hexaghost,
    MonsterEncounter::JawWorm,
    MonsterEncounter::Lagavulin,
    MonsterEncounter::LargeSlime,
    MonsterEncounter::Looter,
    MonsterEncounter::LotsOfSlimes,
    MonsterEncounter::RedSlaver,
    MonsterEncounter::SlimeBoss,
    MonsterEncounter::SmallSlimes,
    MonsterEncounter::TheGuardian,
    MonsterEncounter::ThreeLouse,
    MonsterEncounter::ThreeSentries,
    MonsterEncounter::TwoFungiBeasts,
    MonsterEncounter::TwoLouse,
];
// Assert that all `MonsterEncounter` members are covered
const _: () = assert!(ALL_ENCOUNTERS.len() == MonsterEncounter::COUNT);

// Assert there's no duplicates
const _: () = {
    let mut seen = [false; MonsterEncounter::COUNT];
    let mut idx = 0;
    while idx < ALL_ENCOUNTERS.len() {
        let jdx = ALL_ENCOUNTERS[idx] as usize;
        assert!(!seen[jdx], "ALL_ENCOUNTERS contains a duplicate MonsterEncounter");
        seen[jdx] = true;
        idx += 1;
    }
};


pub const fn get_encounter_pool(e: MonsterEncounter) -> EncounterPool {
    match e {
        // Easy
        MonsterEncounter::Cultist
        | MonsterEncounter::JawWorm
        | MonsterEncounter::TwoLouse
        | MonsterEncounter::SmallSlimes => EncounterPool::Act1Easy,

        // Hard
        MonsterEncounter::BlueSlaver
        | MonsterEncounter::RedSlaver
        | MonsterEncounter::Looter
        | MonsterEncounter::TwoFungiBeasts
        | MonsterEncounter::ThreeLouse
        | MonsterEncounter::LargeSlime
        | MonsterEncounter::LotsOfSlimes
        | MonsterEncounter::GremlinGang
        | MonsterEncounter::ExordiumThugs
        | MonsterEncounter::ExordiumWildlife => EncounterPool::Act1Hard,

        // Elite
        MonsterEncounter::GremlinNob
        | MonsterEncounter::Lagavulin
        | MonsterEncounter::ThreeSentries => EncounterPool::Act1Elite,

        // Boss
        MonsterEncounter::TheGuardian
        | MonsterEncounter::Hexaghost
        | MonsterEncounter::SlimeBoss => EncounterPool::Act1Boss,
    }
}

pub const fn get_encounter_weight(e: MonsterEncounter) -> f32 {
    match e {
        // Easy
        MonsterEncounter::Cultist => 2.0,
        MonsterEncounter::JawWorm => 2.0,
        MonsterEncounter::TwoLouse => 2.0,
        MonsterEncounter::SmallSlimes => 2.0,
        
        // Hard
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
        
        // Elite
        MonsterEncounter::GremlinNob => 1.0,
        MonsterEncounter::Lagavulin => 1.0,
        MonsterEncounter::ThreeSentries => 1.0,
        
        // Boss
        MonsterEncounter::TheGuardian => 1.0,
        MonsterEncounter::Hexaghost => 1.0,
        MonsterEncounter::SlimeBoss => 1.0,
    }
}


const fn pool_eq(a: EncounterPool, b: EncounterPool) -> bool {
    matches!(
        (a, b),
        (EncounterPool::Act1Easy, EncounterPool::Act1Easy)
            | (EncounterPool::Act1Hard, EncounterPool::Act1Hard)
            | (EncounterPool::Act1Elite, EncounterPool::Act1Elite)
            | (EncounterPool::Act1Boss, EncounterPool::Act1Boss)
    )
}

const fn count_pool(pool: EncounterPool) -> usize {
    let mut num = 0;
    let mut idx = 0;
    while idx < ALL_ENCOUNTERS.len() {
        if pool_eq(get_encounter_pool(ALL_ENCOUNTERS[idx]), pool) {
            num += 1;
        }
        idx += 1;
    }
    num
}

const fn build_pool<const N: usize>(pool: EncounterPool) -> [MonsterEncounter; N] {
    // Initialize buffer w/ placeholder encounters
    let mut buf = [MonsterEncounter::Cultist; N];

    let mut idx = 0;
    let mut jdx = 0;
    while idx < ALL_ENCOUNTERS.len() {
        if pool_eq(get_encounter_pool(ALL_ENCOUNTERS[idx]), pool) {
            // Add encounter to buffer
            buf[jdx] = ALL_ENCOUNTERS[idx];
            jdx += 1;
        }
        idx += 1;
    }
    buf
}


// Number of encounters per pool
const NUM_EASY: usize = count_pool(EncounterPool::Act1Easy);
const NUM_HARD: usize = count_pool(EncounterPool::Act1Hard);
const NUM_ELITE: usize = count_pool(EncounterPool::Act1Elite);
const NUM_BOSS: usize = count_pool(EncounterPool::Act1Boss);

// Encounter arrays per pool
const ENC_POOL_EASY: [MonsterEncounter; NUM_EASY] = build_pool(EncounterPool::Act1Easy);
const ENC_POOL_HARD: [MonsterEncounter; NUM_HARD] = build_pool(EncounterPool::Act1Hard);
const ENC_POOL_ELITE: [MonsterEncounter; NUM_ELITE] = build_pool(EncounterPool::Act1Elite);
const ENC_POOL_BOSS: [MonsterEncounter; NUM_BOSS] = build_pool(EncounterPool::Act1Boss);


// Sort ascending by weight (stable for ties), normalize to sum 1.0
fn normalize_weights(pool: &[MonsterEncounter]) -> Vec<(MonsterEncounter, f32)> {
    // Pair each encounter with its raw weight
    let mut encounter_table: Vec<(MonsterEncounter, f32)> =
        pool.iter().map(|&e| (e, get_encounter_weight(e))).collect();
    
    // Stable ascending sort
    encounter_table.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Divide by total so cumulative sum across the table reaches 1.0
    let total: f32 = encounter_table.iter().map(|(_, w)| *w).sum();
    for (_, w) in &mut encounter_table {
        *w /= total;
    }
    encounter_table
}

fn roll_encounter(encounter_table: &[(MonsterEncounter, f32)], value: f32) -> MonsterEncounter {
    let mut weight_cum = 0.0;
    for &(encounter, weight) in encounter_table {
        weight_cum += weight;
        if weight_cum >= value {
            return encounter;
        }
    }
    // Float rounding can leave the final cumulative just below 1.0; return last
    encounter_table.last().unwrap().0
}


fn populate_encounter_list(
    encounter_list: &mut Vec<MonsterEncounter>,
    encounter_table: &[(MonsterEncounter, f32)],
    num: usize,
    elites: bool,
    rng: &mut impl Rng,
) {
    let mut count = 0;
    while count < num {
        if encounter_list.is_empty() {
            // Initial roll
            encounter_list.push(roll_encounter(encounter_table, rng.random_range(0.0..1.0)));
            count += 1;
            continue;
        }

        // Roll candidate
        let encounter_candidate = roll_encounter(encounter_table, rng.random_range(0.0..1.0));

        // If candidate is equal to the last roll, skip
        if encounter_candidate == *encounter_list.last().unwrap() {
            continue;
        }

        // If candidate is equal to second-to-last roll, skip
        if !elites && encounter_list.len() >= 2 && encounter_candidate == encounter_list[encounter_list.len() - 2] {
            continue;
        }

        // Push candidate
        encounter_list.push(encounter_candidate);
        count += 1;
    }
}

fn populate_first_hard_encounter(
    encounter_list: &mut Vec<MonsterEncounter>,
    encounter_table: &[(MonsterEncounter, f32)],
    encounter_exclusions: &[MonsterEncounter],
    rng: &mut impl Rng,
) {
    loop {
        let encounter = roll_encounter(encounter_table, rng.random_range(0.0..1.0));
        if !encounter_exclusions.contains(&encounter) {
            encounter_list.push(encounter);
            return;
        }
    }
}

// Keyed on the last weak entry
fn get_act1_exclusions(encounter_last_weak: MonsterEncounter) -> &'static [MonsterEncounter] {
    match encounter_last_weak {
        MonsterEncounter::TwoLouse => &[MonsterEncounter::ThreeLouse],
        MonsterEncounter::SmallSlimes => {
            &[MonsterEncounter::LargeSlime, MonsterEncounter::LotsOfSlimes]
        }
        _ => &[],
    }
}


pub fn generate_act1_monsters(
    encounter_list: &mut Vec<MonsterEncounter>,
    elite_list: &mut Vec<MonsterEncounter>,
    rng: &mut impl Rng,
) {
    // Get normalized encounter tables for each pool
    let encounter_table_easy = normalize_weights(&ENC_POOL_EASY);
    let encounter_table_hard = normalize_weights(&ENC_POOL_HARD);
    let encounter_table_elite = normalize_weights(&ENC_POOL_ELITE);

    // Sample 3 easy encounters
    populate_encounter_list(encounter_list, &encounter_table_easy, 3, false, rng);

    // Get exclusions based on last easy encounter
    let encounter_exclusions = get_act1_exclusions(*encounter_list.last().unwrap());

    // Populate the first hard encounter
    populate_first_hard_encounter(encounter_list, &encounter_table_hard, encounter_exclusions, rng);

    // Populate rest of hard encounters
    populate_encounter_list(encounter_list, &encounter_table_hard, 12, false, rng);

    // Populate elites
    populate_encounter_list(elite_list, &encounter_table_elite, 10, true, rng);
}

pub fn pick_act1_boss(rng: &mut impl Rng) -> MonsterEncounter {
    const BOSSES: [MonsterEncounter; 3] = [
        MonsterEncounter::TheGuardian,
        MonsterEncounter::Hexaghost,
        MonsterEncounter::SlimeBoss,
    ];
    BOSSES[rng.random_range(0..3)]
}
