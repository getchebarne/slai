use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

// Circle of Power: 3 Strength to every living Monster, Donu included
static MOVE_CIRCLE_OF_POWER: Move = make_move(
    "Circle of Power",
    &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Strength,
            stacks: 3,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    Intent::Buff,
);
static MOVE_BEAM_10: Move = move_attack("Beam", 10, 2);
static MOVE_BEAM_12: Move = move_attack("Beam", 12, 2);

static MOVES_ASC0: [Move; 2] = [MOVE_CIRCLE_OF_POWER, MOVE_BEAM_10];
static MOVES_ASC4: [Move; 2] = [MOVE_CIRCLE_OF_POWER, MOVE_BEAM_12];

const IDX_MOVE_CIRCLE_OF_POWER: usize = 0;
const IDX_MOVE_BEAM: usize = 1;

pub static DONU: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Donu,
    kind: MonsterKind::Boss,
    health_tiers: &[(0, (250, 250)), (9, (265, 265))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES_ASC0]), (4, &[&MOVES_ASC4])],
    modifier_tiers: &[
        (0, &[modifier_fixed(ModifierKind::Artifact, 2)]),
        (19, &[modifier_fixed(ModifierKind::Artifact, 3)]),
    ],
};

// Strict alternation that opens with Circle of Power
pub fn get_next_move_donu(move_current: Option<usize>) -> usize {
    if move_current == Some(IDX_MOVE_CIRCLE_OF_POWER) {
        IDX_MOVE_BEAM
    } else {
        IDX_MOVE_CIRCLE_OF_POWER
    }
}
