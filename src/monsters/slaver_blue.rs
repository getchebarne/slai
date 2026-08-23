use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::monsters::move_attack_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_STAB_12: Move = move_attack("Stab", 12, 1);
static MOVE_STAB_13: Move = move_attack("Stab", 13, 1);
static MOVE_RAKE_7_W1: Move = move_attack_debuff("Rake", 7, ModifierKind::Weak, 1);
static MOVE_RAKE_8_W1: Move = move_attack_debuff("Rake", 8, ModifierKind::Weak, 1);
static MOVE_RAKE_8_W2: Move = move_attack_debuff("Rake", 8, ModifierKind::Weak, 2);
static MOVES_ASC0: [Move; 2] = [MOVE_STAB_12, MOVE_RAKE_7_W1];
static MOVES_ASC2: [Move; 2] = [MOVE_STAB_13, MOVE_RAKE_8_W1];
static MOVES_ASC17: [Move; 2] = [MOVE_STAB_13, MOVE_RAKE_8_W2];

const IDX_MOVE_STAB: usize = 0;
const IDX_MOVE_RAKE: usize = 1;

pub static SLAVER_BLUE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SlaverBlue,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (46, 50)), (7, (48, 52))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[],
};

pub fn get_next_move_slaver_blue(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    let stab_twice = move_history.ends_with(&[IDX_MOVE_STAB as u8, IDX_MOVE_STAB as u8]);
    if roll >= 40 && !stab_twice {
        return IDX_MOVE_STAB;
    }
    if ascension_level >= 17 {
        let rake_last = move_history.last().copied() == Some(IDX_MOVE_RAKE as u8);
        if !rake_last {
            IDX_MOVE_RAKE
        } else {
            IDX_MOVE_STAB
        }
    } else {
        let rake_twice = move_history.ends_with(&[IDX_MOVE_RAKE as u8, IDX_MOVE_RAKE as u8]);
        if !rake_twice {
            IDX_MOVE_RAKE
        } else {
            IDX_MOVE_STAB
        }
    }
}
