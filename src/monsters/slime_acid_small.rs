use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::monsters::move_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_TACKLE_3: Move = move_attack("Tackle", 3, 1);
static MOVE_TACKLE_4: Move = move_attack("Tackle", 4, 1);
static MOVE_LICK: Move = move_debuff("Lick", ModifierKind::Weak, 1, Intent::Debuff);
static MOVES_ASC0: [Move; 2] = [MOVE_TACKLE_3, MOVE_LICK];
static MOVES_ASC2: [Move; 2] = [MOVE_TACKLE_4, MOVE_LICK];
static MOVES_ASC17: [Move; 2] = [MOVE_TACKLE_4, MOVE_LICK];

const IDX_MOVE_TACKLE: usize = 0;
const IDX_MOVE_LICK: usize = 1;

pub static SLIME_ACID_SMALL: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SlimeAcidSmall,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (8, 12)), (7, (9, 13))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[],
};

pub fn get_next_move_slime_acid_small(
    move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        if ascension_level >= 17 {
            return IDX_MOVE_LICK;
        }
        return if rng.random_bool(0.5) {
            IDX_MOVE_TACKLE
        } else {
            IDX_MOVE_LICK
        };
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;
    if last == IDX_MOVE_TACKLE {
        IDX_MOVE_LICK
    } else {
        IDX_MOVE_TACKLE
    }
}
