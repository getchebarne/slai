use crate::entity::Entity;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_SCRATCH_4: Move = make_move_attack("Scratch", 4, 1);
static MOVE_SCRATCH_5: Move = make_move_attack("Scratch", 5, 1);
static MOVES_ASC0: [Move; 1] = [MOVE_SCRATCH_4];
static MOVES_ASC2: [Move; 1] = [MOVE_SCRATCH_5];
static MOVES_ASC17: [Move; 1] = [MOVE_SCRATCH_5];

pub fn spawn_monster_gremlin_warrior(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (20, 24)
    } else {
        (21, 25)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    let angry_stacks: i16 = if ascension_level >= 17 { 2 } else { 1 };
    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Angry, angry_stacks);

    make_entity_monster(
        MonsterName::GremlinWarrior,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        moves,
    )
}
