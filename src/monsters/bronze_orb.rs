use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
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
static MOVE_BEAM: Move = move_attack("Beam", 8, 1);

static MOVES: [Move; 3] = [MOVE_STASIS, MOVE_BEAM, MOVE_SUPPORT_BEAM];

const IDX_MOVE_STASIS: usize = 0;
const IDX_MOVE_BEAM: usize = 1;
const IDX_MOVE_SUPPORT_BEAM: usize = 2;

pub static BRONZE_ORB: MonsterTemplate = MonsterTemplate {
    name: MonsterName::BronzeOrb,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (52, 58)), (9, (54, 60))],
    block_start: 0,
    move_tiers: &[(0, &[&MOVES])],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::Minion, 1)])],
};

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
