use crate::consts::MAX_MONSTERS;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// Defend: block onto a random other Monster
const fn move_defend(block: u16) -> Move {
    make_move(
        "Defend",
        &[Effect {
            kind: EffectKind::BlockGain { amount: block },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters,
                filter: CandidateFilter::NotSource,
                selection_kind: SelectionKind::Random { count: 1 },
            },
        }],
        Intent::Block,
    )
}

static MOVE_SLASH_12: Move = move_attack("Slash", 12, 1);
static MOVE_SLASH_14: Move = move_attack("Slash", 14, 1);
static MOVE_FURY_6: Move = move_attack("Fury", 6, 3);
static MOVE_FURY_7: Move = move_attack("Fury", 7, 3);
static MOVE_DEFEND_15: Move = move_defend(15);
static MOVE_DEFEND_20: Move = move_defend(20);

static MOVES_ASC0: [Move; 3] = [MOVE_SLASH_12, MOVE_FURY_6, MOVE_DEFEND_15];
static MOVES_ASC2: [Move; 3] = [MOVE_SLASH_14, MOVE_FURY_7, MOVE_DEFEND_15];
static MOVES_ASC17: [Move; 3] = [MOVE_SLASH_14, MOVE_FURY_7, MOVE_DEFEND_20];

const IDX_MOVE_SLASH: usize = 0;
const IDX_MOVE_FURY: usize = 1;
const IDX_MOVE_DEFEND: usize = 2;

pub static CENTURION: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Centurion,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (76, 80)), (7, (78, 83))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[],
};

pub fn get_next_move_centurion(
    move_history: &[u8],
    entity_id: usize,
    id_monsters: &[Option<usize>; MAX_MONSTERS],
    rng: &mut impl Rng,
) -> usize {
    // Fury only ever fires once the Centurion stands alone
    let alone = !id_monsters.iter().flatten().any(|&id| id != entity_id);
    let defend_or_fury = if alone {
        IDX_MOVE_FURY
    } else {
        IDX_MOVE_DEFEND
    };

    let roll = rng.random_range(0..=99);
    if (roll >= 65
        && !move_history.ends_with(&[IDX_MOVE_DEFEND as u8, IDX_MOVE_DEFEND as u8])
        && !move_history.ends_with(&[IDX_MOVE_FURY as u8, IDX_MOVE_FURY as u8]))
        || move_history.ends_with(&[IDX_MOVE_SLASH as u8, IDX_MOVE_SLASH as u8])
    {
        defend_or_fury
    } else {
        IDX_MOVE_SLASH
    }
}
