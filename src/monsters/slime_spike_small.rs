use crate::entity::Entity;
use crate::entity::Move;
use crate::modifier::ZERO_MODIFIERS;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_TACKLE_5: Move = make_move_attack("Tackle", 5, 1);
static MOVE_TACKLE_6: Move = make_move_attack("Tackle", 6, 1);
static MOVES_ASC0: [Move; 1] = [MOVE_TACKLE_5];
static MOVES_ASC2: [Move; 1] = [MOVE_TACKLE_6];
static MOVES_ASC17: [Move; 1] = [MOVE_TACKLE_6];

pub fn spawn_monster_slime_spike_small(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (10, 14)
    } else {
        (11, 15)
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
        MonsterName::SlimeSpikeSmall,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        ZERO_MODIFIERS,
        moves,
    )
}
