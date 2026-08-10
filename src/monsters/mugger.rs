use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_SOURCE;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::monsters::make_move_block;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_MUG_10: Move = make_move_attack("Mug", 10, 1);
static MOVE_MUG_11: Move = make_move_attack("Mug", 11, 1);
static MOVE_BIG_SWIPE_16: Move = make_move_attack("Big Swipe", 16, 1);
static MOVE_BIG_SWIPE_18: Move = make_move_attack("Big Swipe", 18, 1);
static MOVE_SMOKE_BOMB_11: Move = make_move_block("Smoke Bomb", 11);
static MOVE_SMOKE_BOMB_17: Move = make_move_block("Smoke Bomb", 17);
static MOVE_ESCAPE: Move = make_move(
    "Escape",
    &[Effect {
        kind: EffectKind::MonsterEscape,
        id_source: None,
        target: TARGET_SOURCE,
    }],
    Intent::Escape,
);

static MOVES_ASC0: [Move; 4] = [
    MOVE_MUG_10,
    MOVE_BIG_SWIPE_16,
    MOVE_SMOKE_BOMB_11,
    MOVE_ESCAPE,
];
static MOVES_ASC2: [Move; 4] = [
    MOVE_MUG_11,
    MOVE_BIG_SWIPE_18,
    MOVE_SMOKE_BOMB_11,
    MOVE_ESCAPE,
];
static MOVES_ASC17: [Move; 4] = [
    MOVE_MUG_11,
    MOVE_BIG_SWIPE_18,
    MOVE_SMOKE_BOMB_17,
    MOVE_ESCAPE,
];

pub fn spawn_monster_mugger(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (48, 52)
    } else {
        (50, 54)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    let stacks_thievery: i16 = if ascension_level < 17 { 15 } else { 20 };
    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Thievery, stacks_thievery);

    make_entity_monster(
        MonsterName::Mugger,
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

// Move order matches the Looter's; the dispatch arm reuses its AI script verbatim
