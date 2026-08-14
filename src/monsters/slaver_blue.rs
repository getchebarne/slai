use crate::entity::Entity;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack;
use crate::monsters::make_move_attack_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_STAB_12: Move = make_move_attack("Stab", 12, 1);
static MOVE_STAB_13: Move = make_move_attack("Stab", 13, 1);
static MOVE_RAKE_7_W1: Move = make_move_attack_debuff("Rake", 7, ModifierKind::Weak, 1);
static MOVE_RAKE_8_W1: Move = make_move_attack_debuff("Rake", 8, ModifierKind::Weak, 1);
static MOVE_RAKE_8_W2: Move = make_move_attack_debuff("Rake", 8, ModifierKind::Weak, 2);
static MOVES_ASC0: [Move; 2] = [MOVE_STAB_12, MOVE_RAKE_7_W1];
static MOVES_ASC2: [Move; 2] = [MOVE_STAB_13, MOVE_RAKE_8_W1];
static MOVES_ASC17: [Move; 2] = [MOVE_STAB_13, MOVE_RAKE_8_W2];

const IDX_MOVE_STAB: usize = 0;
const IDX_MOVE_RAKE: usize = 1;

pub fn spawn_monster_slaver_blue(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (46, 50)
    } else {
        (48, 52)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    make_entity_monster(
        MonsterName::SlaverBlue,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        MODIFIERS_ZERO,
        moves,
    )
}

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
