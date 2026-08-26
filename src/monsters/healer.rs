use crate::consts::MAX_MONSTERS;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack_debuff;
use crate::types::DeltaSign;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

const fn move_heal(amount: u16) -> Move {
    make_move(
        "Heal",
        &[Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(amount),
            },
            id_source: None,
            target: TARGET_MONSTERS_ALL,
        }],
        Intent::Buff,
    )
}

const fn move_buff_all(strength: i16) -> Move {
    make_move(
        "Buff",
        &[Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: strength,
            },
            id_source: None,
            target: TARGET_MONSTERS_ALL,
        }],
        Intent::Buff,
    )
}

static MOVE_STAFF_BASH_8: Move = move_attack_debuff("Staff Bash", 8, ModifierKind::Frail, 2);
static MOVE_STAFF_BASH_9: Move = move_attack_debuff("Staff Bash", 9, ModifierKind::Frail, 2);
static MOVE_HEAL_16: Move = move_heal(16);
static MOVE_HEAL_20: Move = move_heal(20);
static MOVE_BUFF_2: Move = move_buff_all(2);
static MOVE_BUFF_3: Move = move_buff_all(3);
static MOVE_BUFF_4: Move = move_buff_all(4);

static MOVES_ASC0: [Move; 3] = [MOVE_STAFF_BASH_8, MOVE_HEAL_16, MOVE_BUFF_2];
static MOVES_ASC2: [Move; 3] = [MOVE_STAFF_BASH_9, MOVE_HEAL_16, MOVE_BUFF_3];
static MOVES_ASC17: [Move; 3] = [MOVE_STAFF_BASH_9, MOVE_HEAL_20, MOVE_BUFF_4];

const IDX_MOVE_STAFF_BASH: usize = 0;
const IDX_MOVE_HEAL: usize = 1;
const IDX_MOVE_BUFF: usize = 2;

pub static HEALER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Healer,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (48, 56)), (7, (50, 58))],
    block_start: 0,
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (2, &[&MOVES_ASC2]),
        (17, &[&MOVES_ASC17]),
    ],
    modifier_tiers: &[],
};

pub fn get_next_move_healer(
    move_history: &[u8],
    entities: &[Entity],
    id_monsters: &[Option<usize>; MAX_MONSTERS],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let party_hp_deficit: u16 = id_monsters
        .iter()
        .flatten()
        .map(|&id| entities[id].vitals.health_max - entities[id].vitals.health)
        .sum();

    let heal_threshold = if ascension_level >= 17 { 20 } else { 15 };
    if party_hp_deficit > heal_threshold
        && !move_history.ends_with(&[IDX_MOVE_HEAL as u8, IDX_MOVE_HEAL as u8])
    {
        return IDX_MOVE_HEAL;
    }

    // A17+ attacks more often: only one Staff Bash in a row is blocked
    let attack_blocked = if ascension_level >= 17 {
        move_history.last().copied() == Some(IDX_MOVE_STAFF_BASH as u8)
    } else {
        move_history.ends_with(&[IDX_MOVE_STAFF_BASH as u8, IDX_MOVE_STAFF_BASH as u8])
    };
    if (rng.random_range(0..=99) >= 40 && !attack_blocked)
        || move_history.ends_with(&[IDX_MOVE_BUFF as u8, IDX_MOVE_BUFF as u8])
    {
        IDX_MOVE_STAFF_BASH
    } else {
        IDX_MOVE_BUFF
    }
}
