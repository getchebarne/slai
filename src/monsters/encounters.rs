use rand::Rng;
use strum::EnumCount;

use crate::consts::NUM_ENCOUNTERS_EASY;
use crate::consts::NUM_ENCOUNTERS_EASY_ACT2;
use crate::consts::NUM_ENCOUNTERS_ELITE;
use crate::consts::NUM_ENCOUNTERS_HARD;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::monsters::GREMLIN_POOL;
use crate::types::EncounterPool;
use crate::types::MonsterEncounter;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::shuffle;

pub const ALL_ENCOUNTERS: &[MonsterEncounter] = &[
    MonsterEncounter::BlueSlaver,
    MonsterEncounter::ThreeFungiBeasts,
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
    MonsterEncounter::SphericGuardian,
    MonsterEncounter::Chosen,
    MonsterEncounter::ShelledParasite,
    MonsterEncounter::ThreeByrds,
    MonsterEncounter::TwoThieves,
    MonsterEncounter::SnakePlant,
    MonsterEncounter::CenturionAndHealer,
    MonsterEncounter::Snecko,
    MonsterEncounter::CultistAndChosen,
    MonsterEncounter::ThreeCultists,
    MonsterEncounter::ShelledParasiteAndFungi,
    MonsterEncounter::ChosenAndByrds,
    MonsterEncounter::SentryAndSphere,
    MonsterEncounter::GremlinLeader,
    MonsterEncounter::Slavers,
    MonsterEncounter::BookOfStabbing,
    MonsterEncounter::BronzeAutomaton,
    MonsterEncounter::TheCollector,
    MonsterEncounter::Champ,
];
// Assert that all `MonsterEncounter` members are covered
const _: () = assert!(ALL_ENCOUNTERS.len() == MonsterEncounter::COUNT);

// Assert there's no duplicates
const _: () = {
    let mut seen = [false; MonsterEncounter::COUNT];
    let mut idx = 0;
    while idx < ALL_ENCOUNTERS.len() {
        let jdx = ALL_ENCOUNTERS[idx] as usize;
        assert!(
            !seen[jdx],
            "ALL_ENCOUNTERS contains a duplicate MonsterEncounter"
        );
        seen[jdx] = true;
        idx += 1;
    }
};

pub const fn get_encounter_pool(encounter: MonsterEncounter) -> EncounterPool {
    match encounter {
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

        // Act 2 easy
        MonsterEncounter::SphericGuardian
        | MonsterEncounter::Chosen
        | MonsterEncounter::ShelledParasite
        | MonsterEncounter::ThreeByrds
        | MonsterEncounter::TwoThieves => EncounterPool::Act2Easy,

        // Act 2 hard
        MonsterEncounter::SnakePlant
        | MonsterEncounter::CenturionAndHealer
        | MonsterEncounter::Snecko
        | MonsterEncounter::CultistAndChosen
        | MonsterEncounter::ThreeCultists
        | MonsterEncounter::ShelledParasiteAndFungi
        | MonsterEncounter::ChosenAndByrds
        | MonsterEncounter::SentryAndSphere => EncounterPool::Act2Hard,

        // Act 2 elite
        MonsterEncounter::GremlinLeader
        | MonsterEncounter::Slavers
        | MonsterEncounter::BookOfStabbing => EncounterPool::Act2Elite,

        // Act 2 boss
        MonsterEncounter::BronzeAutomaton
        | MonsterEncounter::TheCollector
        | MonsterEncounter::Champ => EncounterPool::Act2Boss,

        // Event-only
        MonsterEncounter::ThreeFungiBeasts => EncounterPool::Event,
    }
}

pub const fn get_encounter_weight(encounter: MonsterEncounter) -> f32 {
    match encounter {
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

        // Act 2 easy
        MonsterEncounter::SphericGuardian => 2.0,
        MonsterEncounter::Chosen => 2.0,
        MonsterEncounter::ShelledParasite => 2.0,
        MonsterEncounter::ThreeByrds => 2.0,
        MonsterEncounter::TwoThieves => 2.0,

        // Act 2 hard
        MonsterEncounter::SnakePlant => 6.0,
        MonsterEncounter::CenturionAndHealer => 6.0,
        MonsterEncounter::Snecko => 4.0,
        MonsterEncounter::CultistAndChosen => 3.0,
        MonsterEncounter::ThreeCultists => 3.0,
        MonsterEncounter::ShelledParasiteAndFungi => 3.0,
        MonsterEncounter::ChosenAndByrds => 2.0,
        MonsterEncounter::SentryAndSphere => 2.0,

        // Act 2 elite
        MonsterEncounter::GremlinLeader => 1.0,
        MonsterEncounter::Slavers => 1.0,
        MonsterEncounter::BookOfStabbing => 1.0,

        // Act 2 boss
        MonsterEncounter::BronzeAutomaton => 1.0,
        MonsterEncounter::TheCollector => 1.0,
        MonsterEncounter::Champ => 1.0,

        // Event-only, never rolled from a pool
        MonsterEncounter::ThreeFungiBeasts => 1.0,
    }
}

const fn pool_eq(lhs: EncounterPool, rhs: EncounterPool) -> bool {
    lhs as u8 == rhs as u8
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
const NUM_EASY2: usize = count_pool(EncounterPool::Act2Easy);
const NUM_HARD2: usize = count_pool(EncounterPool::Act2Hard);
const NUM_ELITE2: usize = count_pool(EncounterPool::Act2Elite);

// Encounter arrays per pool
const ENC_POOL_EASY: [MonsterEncounter; NUM_EASY] = build_pool(EncounterPool::Act1Easy);
const ENC_POOL_HARD: [MonsterEncounter; NUM_HARD] = build_pool(EncounterPool::Act1Hard);
const ENC_POOL_ELITE: [MonsterEncounter; NUM_ELITE] = build_pool(EncounterPool::Act1Elite);
const ENC_POOL_EASY2: [MonsterEncounter; NUM_EASY2] = build_pool(EncounterPool::Act2Easy);
const ENC_POOL_HARD2: [MonsterEncounter; NUM_HARD2] = build_pool(EncounterPool::Act2Hard);
const ENC_POOL_ELITE2: [MonsterEncounter; NUM_ELITE2] = build_pool(EncounterPool::Act2Elite);

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
        if !elites
            && encounter_list.len() >= 2
            && encounter_candidate == encounter_list[encounter_list.len() - 2]
        {
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

// First-hard exclusions, keyed on the act and its last easy entry
fn get_act_exclusions(
    act: u8,
    encounter_last_easy: MonsterEncounter,
) -> &'static [MonsterEncounter] {
    match (act, encounter_last_easy) {
        (1, MonsterEncounter::TwoLouse) => &[MonsterEncounter::ThreeLouse],
        (1, MonsterEncounter::SmallSlimes) => {
            &[MonsterEncounter::LargeSlime, MonsterEncounter::LotsOfSlimes]
        }
        (2, MonsterEncounter::SphericGuardian) => &[MonsterEncounter::SentryAndSphere],
        (2, MonsterEncounter::ThreeByrds) => &[MonsterEncounter::ChosenAndByrds],
        (2, MonsterEncounter::Chosen) => &[
            MonsterEncounter::ChosenAndByrds,
            MonsterEncounter::CultistAndChosen,
        ],
        _ => &[],
    }
}

pub fn generate_act_monsters(
    act: u8,
    encounter_list: &mut Vec<MonsterEncounter>,
    elite_list: &mut Vec<MonsterEncounter>,
    rng: &mut impl Rng,
) {
    // Per-act pools and easy-fight count
    let (pool_easy, pool_hard, pool_elite, num_easy_enc): (
        &[MonsterEncounter],
        &[MonsterEncounter],
        &[MonsterEncounter],
        usize,
    ) = match act {
        1 => (
            &ENC_POOL_EASY,
            &ENC_POOL_HARD,
            &ENC_POOL_ELITE,
            NUM_ENCOUNTERS_EASY,
        ),
        2 => (
            &ENC_POOL_EASY2,
            &ENC_POOL_HARD2,
            &ENC_POOL_ELITE2,
            NUM_ENCOUNTERS_EASY_ACT2,
        ),
        _ => unreachable!("no encounter pools for act {act}"),
    };

    // Get normalized encounter tables for each pool
    let encounter_table_easy = normalize_weights(pool_easy);
    let encounter_table_hard = normalize_weights(pool_hard);
    let encounter_table_elite = normalize_weights(pool_elite);

    // Sample easy encounters
    populate_encounter_list(
        encounter_list,
        &encounter_table_easy,
        num_easy_enc,
        false,
        rng,
    );

    // Get exclusions based on last easy encounter
    let encounter_exclusions = get_act_exclusions(act, *encounter_list.last().unwrap());

    // Populate the first hard encounter
    populate_first_hard_encounter(
        encounter_list,
        &encounter_table_hard,
        encounter_exclusions,
        rng,
    );

    // Populate rest of hard encounters
    populate_encounter_list(
        encounter_list,
        &encounter_table_hard,
        NUM_ENCOUNTERS_HARD,
        false,
        rng,
    );

    // Populate elites
    populate_encounter_list(
        elite_list,
        &encounter_table_elite,
        NUM_ENCOUNTERS_ELITE,
        true,
        rng,
    );
}

// Boss arrays stay hand-ordered per act: deriving them from ALL_ENCOUNTERS would
// reorder the draw and shift the RNG stream
pub fn pick_boss(act: u8, rng: &mut impl Rng) -> MonsterEncounter {
    const BOSSES_ACT1: [MonsterEncounter; 3] = [
        MonsterEncounter::TheGuardian,
        MonsterEncounter::Hexaghost,
        MonsterEncounter::SlimeBoss,
    ];
    const BOSSES_ACT2: [MonsterEncounter; 3] = [
        MonsterEncounter::BronzeAutomaton,
        MonsterEncounter::TheCollector,
        MonsterEncounter::Champ,
    ];
    let pool: &[MonsterEncounter] = match act {
        1 => &BOSSES_ACT1,
        2 => &BOSSES_ACT2,
        _ => unreachable!("no bosses for act {act}"),
    };
    pool[rng.random_range(0..pool.len())]
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

fn pick_wildlife_weak(rng: &mut impl Rng) -> MonsterName {
    match rng.random_range(0..3) {
        0 => pick_louse(rng),
        1 => MonsterName::SlimeSpikeMedium,
        2 => MonsterName::SlimeAcidMedium,
        _ => unreachable!(),
    }
}

fn pick_wildlife_strong(rng: &mut impl Rng) -> MonsterName {
    if rng.random_bool(0.5) {
        MonsterName::FungiBeast
    } else {
        MonsterName::JawWorm
    }
}

fn pick_humanoid_strong(rng: &mut impl Rng) -> MonsterName {
    match rng.random_range(0..3) {
        0 => MonsterName::Cultist,
        1 => pick_slaver(rng),
        2 => MonsterName::Looter,
        _ => unreachable!(),
    }
}

fn push_monster_spawn(effects: &mut Vec<Effect>, name: MonsterName) {
    effects.push(Effect {
        kind: EffectKind::MonsterSpawn {
            name,
            minion: false,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}

// Queues the encounter's spawns followed by `EffectKind::CombatStart`
pub fn spawn_encounter_monsters(
    state: &mut GameState,
    encounter: MonsterEncounter,
    event_gold: Option<Amount>,
    event_relic: Option<RelicName>,
    event_relic_roll: bool,
) {
    state.effect_buf.clear();
    let effects = &mut state.effect_buf;
    let rng = &mut state.rng;
    match encounter {
        MonsterEncounter::Cultist => push_monster_spawn(effects, MonsterName::Cultist),
        MonsterEncounter::JawWorm => push_monster_spawn(effects, MonsterName::JawWorm),
        MonsterEncounter::TwoLouse => {
            for _ in 0..2 {
                push_monster_spawn(effects, pick_louse(rng));
            }
        }
        MonsterEncounter::SmallSlimes => {
            let (small, medium) = if rng.random_bool(0.5) {
                (MonsterName::SlimeSpikeSmall, MonsterName::SlimeAcidMedium)
            } else {
                (MonsterName::SlimeAcidSmall, MonsterName::SlimeSpikeMedium)
            };
            push_monster_spawn(effects, small);
            push_monster_spawn(effects, medium);
        }
        MonsterEncounter::BlueSlaver => push_monster_spawn(effects, MonsterName::SlaverBlue),
        MonsterEncounter::RedSlaver => push_monster_spawn(effects, MonsterName::SlaverRed),
        MonsterEncounter::Looter => push_monster_spawn(effects, MonsterName::Looter),
        MonsterEncounter::TwoFungiBeasts => {
            push_monster_spawn(effects, MonsterName::FungiBeast);
            push_monster_spawn(effects, MonsterName::FungiBeast);
        }
        MonsterEncounter::ThreeLouse => {
            for _ in 0..3 {
                push_monster_spawn(effects, pick_louse(rng));
            }
        }
        MonsterEncounter::LargeSlime => {
            let name = if rng.random_bool(0.5) {
                MonsterName::SlimeAcidLarge
            } else {
                MonsterName::SlimeSpikeLarge
            };
            push_monster_spawn(effects, name);
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
                push_monster_spawn(effects, name);
            }
        }
        MonsterEncounter::GremlinGang => {
            let mut pool = GREMLIN_POOL;
            shuffle(&mut pool, rng);
            for &name in &pool[..4] {
                push_monster_spawn(effects, name);
            }
        }
        MonsterEncounter::ExordiumThugs => {
            push_monster_spawn(effects, pick_wildlife_weak(rng));
            push_monster_spawn(effects, pick_humanoid_strong(rng));
        }
        MonsterEncounter::ExordiumWildlife => {
            push_monster_spawn(effects, pick_wildlife_strong(rng));
            push_monster_spawn(effects, pick_wildlife_weak(rng));
        }
        MonsterEncounter::GremlinNob => push_monster_spawn(effects, MonsterName::GremlinNob),
        MonsterEncounter::Lagavulin => push_monster_spawn(effects, MonsterName::Lagavulin),
        MonsterEncounter::ThreeSentries => {
            for _ in 0..3 {
                push_monster_spawn(effects, MonsterName::Sentry);
            }
        }
        MonsterEncounter::TheGuardian => push_monster_spawn(effects, MonsterName::TheGuardian),
        MonsterEncounter::Hexaghost => push_monster_spawn(effects, MonsterName::Hexaghost),
        MonsterEncounter::SlimeBoss => push_monster_spawn(effects, MonsterName::SlimeBoss),
        MonsterEncounter::ThreeFungiBeasts => {
            for _ in 0..3 {
                push_monster_spawn(effects, MonsterName::FungiBeast);
            }
        }
        MonsterEncounter::SphericGuardian => {
            push_monster_spawn(effects, MonsterName::SphericGuardian)
        }
        MonsterEncounter::Chosen => push_monster_spawn(effects, MonsterName::Chosen),
        MonsterEncounter::ShelledParasite => {
            push_monster_spawn(effects, MonsterName::ShelledParasite)
        }
        MonsterEncounter::ThreeByrds => {
            for _ in 0..3 {
                push_monster_spawn(effects, MonsterName::Byrd);
            }
        }
        MonsterEncounter::TwoThieves => {
            push_monster_spawn(effects, MonsterName::Looter);
            push_monster_spawn(effects, MonsterName::Mugger);
        }
        MonsterEncounter::SnakePlant => push_monster_spawn(effects, MonsterName::SnakePlant),
        MonsterEncounter::CenturionAndHealer => {
            push_monster_spawn(effects, MonsterName::Centurion);
            push_monster_spawn(effects, MonsterName::Healer);
        }
        MonsterEncounter::Snecko => push_monster_spawn(effects, MonsterName::Snecko),
        MonsterEncounter::CultistAndChosen => {
            push_monster_spawn(effects, MonsterName::Cultist);
            push_monster_spawn(effects, MonsterName::Chosen);
        }
        MonsterEncounter::ThreeCultists => {
            for _ in 0..3 {
                push_monster_spawn(effects, MonsterName::Cultist);
            }
        }
        MonsterEncounter::ShelledParasiteAndFungi => {
            push_monster_spawn(effects, MonsterName::ShelledParasite);
            push_monster_spawn(effects, MonsterName::FungiBeast);
        }
        MonsterEncounter::ChosenAndByrds => {
            push_monster_spawn(effects, MonsterName::Byrd);
            push_monster_spawn(effects, MonsterName::Chosen);
        }
        MonsterEncounter::SentryAndSphere => {
            push_monster_spawn(effects, MonsterName::Sentry);
            push_monster_spawn(effects, MonsterName::SphericGuardian);
        }
        MonsterEncounter::GremlinLeader => {
            // Two weighted gremlins spawn as Minions, then the leader
            for _ in 0..2 {
                effects.push(Effect {
                    kind: EffectKind::GremlinSummon,
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
            push_monster_spawn(effects, MonsterName::GremlinLeader);
        }
        MonsterEncounter::Slavers => {
            push_monster_spawn(effects, MonsterName::SlaverBlue);
            push_monster_spawn(effects, MonsterName::Taskmaster);
            push_monster_spawn(effects, MonsterName::SlaverRed);
        }
        MonsterEncounter::BookOfStabbing => {
            push_monster_spawn(effects, MonsterName::BookOfStabbing)
        }
        MonsterEncounter::BronzeAutomaton => {
            push_monster_spawn(effects, MonsterName::BronzeAutomaton)
        }
        MonsterEncounter::TheCollector => push_monster_spawn(effects, MonsterName::TheCollector),
        MonsterEncounter::Champ => push_monster_spawn(effects, MonsterName::Champ),
    }

    effects.push(Effect {
        kind: EffectKind::CombatStart {
            event_gold,
            event_relic,
            event_relic_roll,
        },
        id_source: None,
        target: Target::Direct(None),
    });
    flush_effects_from_buf_to_queue_front(state);
}
