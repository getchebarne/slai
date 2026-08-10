use crate::entity::Entity;
use crate::entity::Move;
use crate::modifier::ZERO_MODIFIERS;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;

static MOVES_ASC0: [Move; 1] = [make_move_attack("Spit Web", 5, 2)];
static MOVES_ASC2: [Move; 1] = [make_move_attack("Spit Web", 6, 2)];

pub fn spawn_monster_bandit_pointy(ascension_level: u8) -> Entity {
    let health_max = if ascension_level < 7 { 30 } else { 34 };

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else {
        &MOVES_ASC2
    };

    make_entity_monster(
        MonsterName::BanditPointy,
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
