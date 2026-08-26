use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::monsters::move_split;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_GOOP_SPRAY_3: Move = make_move(
    "Goop Spray",
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Slimed,
            pile: CardPile::Discard,
            count: 3,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    Intent::DebuffPowerful,
);
static MOVE_GOOP_SPRAY_5: Move = make_move(
    "Goop Spray",
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Slimed,
            pile: CardPile::Discard,
            count: 5,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    Intent::DebuffPowerful,
);
static MOVE_PREPARING: Move = make_move("Preparing", &[], Intent::Unknown);
static MOVE_SLAM_35: Move = move_attack("Slam", 35, 1);
static MOVE_SLAM_38: Move = move_attack("Slam", 38, 1);
static MOVE_SPLIT: Move = move_split(
    "Split",
    MonsterName::SlimeSpikeLarge,
    MonsterName::SlimeAcidLarge,
);

static MOVES_ASC0: [Move; 4] = [MOVE_GOOP_SPRAY_3, MOVE_PREPARING, MOVE_SLAM_35, MOVE_SPLIT];
static MOVES_ASC4: [Move; 4] = [MOVE_GOOP_SPRAY_3, MOVE_PREPARING, MOVE_SLAM_38, MOVE_SPLIT];
static MOVES_ASC19: [Move; 4] = [MOVE_GOOP_SPRAY_5, MOVE_PREPARING, MOVE_SLAM_38, MOVE_SPLIT];

const IDX_MOVE_GOOP_SPRAY: usize = 0;
const IDX_MOVE_PREPARING: usize = 1;
const IDX_MOVE_SLAM: usize = 2;
pub const IDX_MOVE_SPLIT: usize = 3;

pub static SLIME_BOSS: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SlimeBoss,
    kind: MonsterKind::Boss,
    health_tiers: &[(0, (140, 140)), (9, (150, 150))],
    block_start: 0,
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (4, &[&MOVES_ASC4]),
        (19, &[&MOVES_ASC19]),
    ],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::Splittable, 1)])],
};

pub fn get_next_move_slime_boss(move_current: Option<usize>, move_history: &[u8]) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_GOOP_SPRAY;
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;
    match last {
        IDX_MOVE_GOOP_SPRAY => IDX_MOVE_PREPARING,
        IDX_MOVE_PREPARING => IDX_MOVE_SLAM,
        IDX_MOVE_SLAM => IDX_MOVE_GOOP_SPRAY,
        IDX_MOVE_SPLIT => IDX_MOVE_SPLIT,
        _ => unreachable!("SlimeBoss unexpected move idx: {last}"),
    }
}
