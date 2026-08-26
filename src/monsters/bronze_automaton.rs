use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::monsters::move_block_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_SPAWN_ORBS: Move = make_move(
    "Spawn Orbs",
    &[
        Effect {
            kind: EffectKind::MonsterSpawn {
                name: MonsterName::BronzeOrb,
                minion: false,
                cap: None,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::MonsterSpawn {
                name: MonsterName::BronzeOrb,
                minion: false,
                cap: None,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    Intent::Unknown,
);
static MOVE_STUNNED: Move = make_move("Stunned", &[], Intent::Stunned);
static MOVE_FLAIL_7: Move = move_attack("Flail", 7, 2);
static MOVE_FLAIL_8: Move = move_attack("Flail", 8, 2);
static MOVE_HYPER_BEAM_45: Move = move_attack("Hyper Beam", 45, 1);
static MOVE_HYPER_BEAM_50: Move = move_attack("Hyper Beam", 50, 1);
static MOVE_BOOST_9_3: Move = move_block_buff("Boost", 9, 3);
static MOVE_BOOST_9_4: Move = move_block_buff("Boost", 9, 4);
static MOVE_BOOST_12_4: Move = move_block_buff("Boost", 12, 4);

static MOVES_ASC0: [Move; 5] = [
    MOVE_SPAWN_ORBS,
    MOVE_FLAIL_7,
    MOVE_BOOST_9_3,
    MOVE_HYPER_BEAM_45,
    MOVE_STUNNED,
];
static MOVES_ASC4: [Move; 5] = [
    MOVE_SPAWN_ORBS,
    MOVE_FLAIL_8,
    MOVE_BOOST_9_4,
    MOVE_HYPER_BEAM_50,
    MOVE_STUNNED,
];
static MOVES_ASC9: [Move; 5] = [
    MOVE_SPAWN_ORBS,
    MOVE_FLAIL_8,
    MOVE_BOOST_12_4,
    MOVE_HYPER_BEAM_50,
    MOVE_STUNNED,
];

const IDX_MOVE_SPAWN_ORBS: usize = 0;
const IDX_MOVE_FLAIL: usize = 1;
const IDX_MOVE_BOOST: usize = 2;
const IDX_MOVE_HYPER_BEAM: usize = 3;
const IDX_MOVE_STUNNED: usize = 4;

pub static BRONZE_AUTOMATON: MonsterTemplate = MonsterTemplate {
    name: MonsterName::BronzeAutomaton,
    kind: MonsterKind::Boss,
    health_tiers: &[(0, (300, 300)), (9, (320, 320))],
    block_start: 0,
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (4, &[&MOVES_ASC4]),
        (9, &[&MOVES_ASC9]),
    ],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::Artifact, 3)])],
};

// Deterministic: Spawn, then Flail / Boost alternating; the fifth cycling turn
// is Hyper Beam, followed by a Stunned turn
pub fn get_next_move_bronze_automaton(move_history: &[u8], ascension_level: u8) -> usize {
    if move_history.is_empty() {
        return IDX_MOVE_SPAWN_ORBS;
    }
    let last = *move_history.last().unwrap() as usize;

    // Count cycling turns (Flail / Boost) since the last Hyper Beam
    let segment_start = move_history
        .iter()
        .rposition(|&idx_move| idx_move as usize == IDX_MOVE_HYPER_BEAM)
        // The A19+ post-beam Boost is forced, so it is skipped in the count
        .map_or(0, |pos| {
            if ascension_level >= 19 {
                pos + 2
            } else {
                pos + 1
            }
        });
    let num_turns = move_history
        .iter()
        .skip(segment_start)
        .filter(|&&m| m as usize == IDX_MOVE_FLAIL || m as usize == IDX_MOVE_BOOST)
        .count();

    if num_turns == 4 {
        IDX_MOVE_HYPER_BEAM
    } else if last == IDX_MOVE_HYPER_BEAM {
        if ascension_level >= 19 {
            IDX_MOVE_BOOST
        } else {
            IDX_MOVE_STUNNED
        }
    } else if last == IDX_MOVE_STUNNED || last == IDX_MOVE_BOOST || last == IDX_MOVE_SPAWN_ORBS {
        IDX_MOVE_FLAIL
    } else {
        IDX_MOVE_BOOST
    }
}
