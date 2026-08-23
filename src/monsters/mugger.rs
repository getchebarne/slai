use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_SOURCE;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack;
use crate::monsters::move_block;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_MUG_10: Move = move_attack("Mug", 10, 1);
static MOVE_MUG_11: Move = move_attack("Mug", 11, 1);
static MOVE_BIG_SWIPE_16: Move = move_attack("Big Swipe", 16, 1);
static MOVE_BIG_SWIPE_18: Move = move_attack("Big Swipe", 18, 1);
static MOVE_SMOKE_BOMB_11: Move = move_block("Smoke Bomb", 11);
static MOVE_SMOKE_BOMB_17: Move = move_block("Smoke Bomb", 17);
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

pub static MUGGER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Mugger,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (48, 52)), (7, (50, 54))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[
        (0, &[(ModifierKind::Thievery, 15)]),
        (17, &[(ModifierKind::Thievery, 20)]),
    ],
};

// Move order matches the Looter's; the dispatch arm reuses its AI script verbatim
