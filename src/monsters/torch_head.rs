use crate::entity::Entity;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVES: [Move; 1] = [make_move_attack("Tackle", 7, 1)];

pub fn spawn_monster_torch_head(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 9 {
        (38, 40)
    } else {
        (40, 45)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let mut modifiers = MODIFIERS_ZERO;
    modifier_apply(&mut modifiers, ModifierKind::Minion, 1);

    make_entity_monster(
        MonsterName::TorchHead,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        &MOVES,
    )
}

// Deterministic: always uses Tackle
