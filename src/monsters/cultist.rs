use crate::entity::Entity;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack;
use crate::monsters::make_move_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_DARK_STRIKE: Move = make_move_attack("Dark Strike", 6, 1);
static MOVE_INCANTATION_3: Move = make_move_buff("Incantation", ModifierKind::Ritual, 3);
static MOVE_INCANTATION_4: Move = make_move_buff("Incantation", ModifierKind::Ritual, 4);
static MOVE_INCANTATION_5: Move = make_move_buff("Incantation", ModifierKind::Ritual, 5);
static MOVES_ASC0: [Move; 2] = [MOVE_INCANTATION_3, MOVE_DARK_STRIKE];
static MOVES_ASC2: [Move; 2] = [MOVE_INCANTATION_4, MOVE_DARK_STRIKE];
static MOVES_ASC17: [Move; 2] = [MOVE_INCANTATION_5, MOVE_DARK_STRIKE];

const IDX_MOVE_INCANTATION: usize = 0;
const IDX_MOVE_DARK_STRIKE: usize = 1;

pub fn spawn_monster_cultist(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (48, 54)
    } else {
        (50, 56)
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
        MonsterName::Cultist,
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

pub fn get_next_move_cultist(move_current: Option<usize>) -> usize {
    if move_current.is_none() {
        IDX_MOVE_INCANTATION
    } else {
        IDX_MOVE_DARK_STRIKE
    }
}
