use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::monsters::make_move_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// Defensive Stance: block plus Metallicize
const fn make_move_defensive_stance(block: u16, metallicize: i16) -> Move {
    make_move(
        "Defensive Stance",
        &[
            Effect {
                kind: EffectKind::BlockGain { amount: block },
                id_source: None,
                target: TARGET_SOURCE,
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Metallicize,
                    stacks: metallicize,
                },
                id_source: None,
                target: TARGET_SOURCE,
            },
        ],
        Intent::BlockBuff,
    )
}

// Face Slap: damage plus Frail and Vulnerable
const fn make_move_face_slap(damage: u16) -> Move {
    make_move(
        "Face Slap",
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    lifesteal: false,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Frail,
                    stacks: 2,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Vulnerable,
                    stacks: 2,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
        ],
        Intent::AttackDebuff {
            damage,
            instances: 1,
        },
    )
}

// Taunt: Weak and Vulnerable
static MOVE_TAUNT: Move = make_move(
    "Taunt",
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    Intent::Debuff,
);

// Anger: shed every debuff and surge Strength
const fn make_move_anger(strength: i16) -> Move {
    make_move(
        "Anger",
        &[
            Effect {
                kind: EffectKind::DebuffsClear,
                id_source: None,
                target: TARGET_SOURCE,
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: strength,
                },
                id_source: None,
                target: TARGET_SOURCE,
            },
        ],
        Intent::Buff,
    )
}

static MOVE_HEAVY_SLASH_16: Move = make_move_attack("Heavy Slash", 16, 1);
static MOVE_HEAVY_SLASH_18: Move = make_move_attack("Heavy Slash", 18, 1);
static MOVE_EXECUTE: Move = make_move_attack("Execute", 10, 2);

static MOVES_ASC0: [Move; 7] = [
    MOVE_HEAVY_SLASH_16,
    make_move_defensive_stance(15, 5),
    MOVE_EXECUTE,
    make_move_face_slap(12),
    make_move_buff("Gloat", ModifierKind::Strength, 2),
    MOVE_TAUNT,
    make_move_anger(6),
];
static MOVES_ASC4: [Move; 7] = [
    MOVE_HEAVY_SLASH_18,
    make_move_defensive_stance(15, 5),
    MOVE_EXECUTE,
    make_move_face_slap(14),
    make_move_buff("Gloat", ModifierKind::Strength, 3),
    MOVE_TAUNT,
    make_move_anger(9),
];
static MOVES_ASC9: [Move; 7] = [
    MOVE_HEAVY_SLASH_18,
    make_move_defensive_stance(18, 6),
    MOVE_EXECUTE,
    make_move_face_slap(14),
    make_move_buff("Gloat", ModifierKind::Strength, 3),
    MOVE_TAUNT,
    make_move_anger(9),
];
static MOVES_ASC19: [Move; 7] = [
    MOVE_HEAVY_SLASH_18,
    make_move_defensive_stance(20, 7),
    MOVE_EXECUTE,
    make_move_face_slap(14),
    make_move_buff("Gloat", ModifierKind::Strength, 4),
    MOVE_TAUNT,
    make_move_anger(12),
];

const IDX_MOVE_HEAVY_SLASH: usize = 0;
const IDX_MOVE_DEFENSIVE_STANCE: usize = 1;
const IDX_MOVE_EXECUTE: usize = 2;
const IDX_MOVE_FACE_SLAP: usize = 3;
const IDX_MOVE_GLOAT: usize = 4;
const IDX_MOVE_TAUNT: usize = 5;
const IDX_MOVE_ANGER: usize = 6;

pub fn spawn_monster_champ(ascension_level: u8) -> Entity {
    let health_max = if ascension_level < 9 { 420 } else { 440 };

    let moves: &'static [Move] = if ascension_level < 4 {
        &MOVES_ASC0
    } else if ascension_level < 9 {
        &MOVES_ASC4
    } else if ascension_level < 19 {
        &MOVES_ASC9
    } else {
        &MOVES_ASC19
    };

    make_entity_monster(
        MonsterName::Champ,
        MonsterKind::Boss,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        MODIFIERS_ZERO,
        moves,
    )
}

pub fn get_next_move_champ(
    move_history: &[u8],
    health: u16,
    health_max: u16,
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let threshold_reached = move_history.iter().any(|&m| m as usize == IDX_MOVE_ANGER);

    // Below half HP the Champ rages once, then Executes every chance it gets
    if health < health_max / 2 && !threshold_reached {
        return IDX_MOVE_ANGER;
    }
    if threshold_reached
        && !move_history
            .iter()
            .rev()
            .take(2)
            .any(|&m| m as usize == IDX_MOVE_EXECUTE)
    {
        return IDX_MOVE_EXECUTE;
    }

    // Pre-threshold, Taunt lands every fourth turn
    let turns_since_taunt = move_history
        .iter()
        .rposition(|&m| m as usize == IDX_MOVE_TAUNT)
        .map_or(move_history.len(), |p| move_history.len() - p - 1);
    if !threshold_reached && turns_since_taunt == 3 {
        return IDX_MOVE_TAUNT;
    }

    let last = move_history.last().copied().map(|m| m as usize);
    let roll = rng.random_range(0..=99);
    let forge_roll_max = if ascension_level >= 19 { 30 } else { 15 };
    let forge_times = move_history
        .iter()
        .filter(|&&m| m as usize == IDX_MOVE_DEFENSIVE_STANCE)
        .count();

    if last != Some(IDX_MOVE_DEFENSIVE_STANCE) && forge_times < 2 && roll <= forge_roll_max {
        IDX_MOVE_DEFENSIVE_STANCE
    } else if last != Some(IDX_MOVE_GLOAT) && last != Some(IDX_MOVE_DEFENSIVE_STANCE) && roll <= 30
    {
        IDX_MOVE_GLOAT
    } else if last != Some(IDX_MOVE_FACE_SLAP) && roll <= 55 {
        IDX_MOVE_FACE_SLAP
    } else if last == Some(IDX_MOVE_HEAVY_SLASH) {
        IDX_MOVE_FACE_SLAP
    } else {
        IDX_MOVE_HEAVY_SLASH
    }
}
