use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_STASIS: Move = make_move(
    "Stasis",
    &[Effect {
        kind: EffectKind::StasisSteal,
        id_source: None,
        target: Target::Direct(None),
    }],
    Intent::DebuffPowerful,
);

// Support Beam: block onto the Automaton (the one non-Minion on the field)
static MOVE_SUPPORT_BEAM: Move = make_move(
    "Support Beam",
    &[Effect {
        kind: EffectKind::BlockGain { amount: 12 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters,
            filter: CandidateFilter::NotMinion,
            selection_kind: SelectionKind::All,
        },
    }],
    Intent::Block,
);
static MOVE_BEAM: Move = make_move_attack("Beam", 8, 1);

static MOVES: [Move; 3] = [MOVE_STASIS, MOVE_BEAM, MOVE_SUPPORT_BEAM];

const IDX_MOVE_STASIS: usize = 0;
const IDX_MOVE_BEAM: usize = 1;
const IDX_MOVE_SUPPORT_BEAM: usize = 2;

pub fn spawn_monster_bronze_orb(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 9 {
        (52, 58)
    } else {
        (54, 60)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let mut modifiers = MODIFIERS_ZERO;
    modifier_apply(&mut modifiers, ModifierKind::Minion, 1);

    make_entity_monster(
        MonsterName::BronzeOrb,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        &MOVES,
    )
}

pub fn get_next_move_bronze_orb(move_history: &[u8], rng: &mut impl Rng) -> usize {
    let used_stasis = move_history
        .iter()
        .any(|&idx_move| idx_move as usize == IDX_MOVE_STASIS);
    let roll = rng.random_range(0..=99);

    if !used_stasis && roll >= 25 {
        IDX_MOVE_STASIS
    } else if roll >= 70
        && !move_history.ends_with(&[IDX_MOVE_SUPPORT_BEAM as u8, IDX_MOVE_SUPPORT_BEAM as u8])
    {
        IDX_MOVE_SUPPORT_BEAM
    } else if move_history.ends_with(&[IDX_MOVE_BEAM as u8, IDX_MOVE_BEAM as u8]) {
        IDX_MOVE_SUPPORT_BEAM
    } else {
        IDX_MOVE_BEAM
    }
}
