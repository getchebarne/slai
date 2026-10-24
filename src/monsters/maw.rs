use crate::consts::MAX_MONSTER_MOVES;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack;
use crate::monsters::move_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// Roar: Weak lands ahead of Frail, so a lone Artifact stack negates the Weak
const fn move_roar(stacks: i16) -> Move {
    make_move(
        "Roar",
        &[
            Effect {
                kind: EffectKind::ModifierDelta {
                    kind: ModifierKind::Weak,
                    stacks,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierDelta {
                    kind: ModifierKind::Frail,
                    stacks,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
        ],
        Intent::DebuffPowerful,
    )
}

static MOVE_ROAR_3: Move = move_roar(3);
static MOVE_ROAR_5: Move = move_roar(5);
static MOVE_SLAM_25: Move = move_attack("Slam", 25, 1);
static MOVE_SLAM_30: Move = move_attack("Slam", 30, 1);
static MOVE_DROOL_3: Move = move_buff("Drool", ModifierKind::Strength, 3);
static MOVE_DROOL_5: Move = move_buff("Drool", ModifierKind::Strength, 5);

// Nom's hit count is a placeholder, locked in when the move is chosen
static MOVE_NOM: Move = move_attack("Nom", 5, 0);

static MOVES_ASC0: [Move; 4] = [MOVE_ROAR_3, MOVE_SLAM_25, MOVE_DROOL_3, MOVE_NOM];
static MOVES_ASC2: [Move; 4] = [MOVE_ROAR_3, MOVE_SLAM_30, MOVE_DROOL_3, MOVE_NOM];
static MOVES_ASC17: [Move; 4] = [MOVE_ROAR_5, MOVE_SLAM_30, MOVE_DROOL_5, MOVE_NOM];

const IDX_MOVE_ROAR: usize = 0;
const IDX_MOVE_SLAM: usize = 1;
const IDX_MOVE_DROOL: usize = 2;
pub const IDX_MOVE_NOM: usize = 3;

pub static MAW: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Maw,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (300, 300))],
    block_tiers: &[],
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (2, &[&MOVES_ASC2]),
        (17, &[&MOVES_ASC17]),
    ],
    modifier_tiers: &[],
};

pub fn get_next_move_maw(
    move_current: Option<usize>,
    move_history: &[u8],
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_ROAR;
    }
    let last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    if last != Some(IDX_MOVE_NOM) && rng.random_range(0..=99) < 50 {
        IDX_MOVE_NOM
    } else if last == Some(IDX_MOVE_SLAM) || last == Some(IDX_MOVE_NOM) {
        IDX_MOVE_DROOL
    } else {
        IDX_MOVE_SLAM
    }
}

// Nom as the Maw's k-th move hits (k + 1) / 2 times
pub fn nom_instances(move_uses: &[u8; MAX_MONSTER_MOVES]) -> u8 {
    let moves_chosen_before: u16 = move_uses.iter().map(|&uses| u16::from(uses)).sum();
    ((moves_chosen_before + 2) / 2).min(u8::MAX as u16) as u8
}
