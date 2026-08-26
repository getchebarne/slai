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

// Protect: block onto a random other Monster
const fn move_protect(block: u16) -> Move {
    make_move(
        "Protect",
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

static MOVE_PROTECT_7: Move = move_protect(7);
static MOVE_PROTECT_8: Move = move_protect(8);
static MOVE_PROTECT_11: Move = move_protect(11);
static MOVE_BASH_6: Move = move_attack("Shield Bash", 6, 1);
static MOVE_BASH_8: Move = move_attack("Shield Bash", 8, 1);

static MOVES_ASC0: [Move; 2] = [MOVE_PROTECT_7, MOVE_BASH_6];
static MOVES_ASC2: [Move; 2] = [MOVE_PROTECT_7, MOVE_BASH_8];
static MOVES_ASC7: [Move; 2] = [MOVE_PROTECT_8, MOVE_BASH_8];
static MOVES_ASC17: [Move; 2] = [MOVE_PROTECT_11, MOVE_BASH_8];

const IDX_MOVE_PROTECT: usize = 0;
const IDX_MOVE_BASH: usize = 1;

pub static GREMLIN_TSUNDERE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::GremlinTsundere,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (12, 15)), (7, (13, 17))],
    block_start: 0,
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (2, &[&MOVES_ASC2]),
        (7, &[&MOVES_ASC7]),
        (17, &[&MOVES_ASC17]),
    ],
    modifier_tiers: &[],
};

pub fn get_next_move_gremlin_tsundere(move_current: Option<usize>, other_alive_count: u8) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_PROTECT;
    }
    let last = move_current.unwrap();
    if last == IDX_MOVE_BASH {
        return IDX_MOVE_BASH;
    }
    // Last was Protect
    if other_alive_count > 0 {
        IDX_MOVE_PROTECT
    } else {
        IDX_MOVE_BASH
    }
}
