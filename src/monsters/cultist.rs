use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::monsters::move_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_DARK_STRIKE: Move = move_attack("Dark Strike", 6, 1);
static MOVE_INCANTATION_3: Move = move_buff("Incantation", ModifierKind::Ritual, 3);
static MOVE_INCANTATION_4: Move = move_buff("Incantation", ModifierKind::Ritual, 4);
static MOVE_INCANTATION_5: Move = move_buff("Incantation", ModifierKind::Ritual, 5);
static MOVES_ASC0: [Move; 2] = [MOVE_INCANTATION_3, MOVE_DARK_STRIKE];
static MOVES_ASC2: [Move; 2] = [MOVE_INCANTATION_4, MOVE_DARK_STRIKE];
static MOVES_ASC17: [Move; 2] = [MOVE_INCANTATION_5, MOVE_DARK_STRIKE];

const IDX_MOVE_INCANTATION: usize = 0;
const IDX_MOVE_DARK_STRIKE: usize = 1;

pub static CULTIST: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Cultist,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (48, 54)), (7, (50, 56))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[],
};

pub fn get_next_move_cultist(move_current: Option<usize>) -> usize {
    if move_current.is_none() {
        IDX_MOVE_INCANTATION
    } else {
        IDX_MOVE_DARK_STRIKE
    }
}
