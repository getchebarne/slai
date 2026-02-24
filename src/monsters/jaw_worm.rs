use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::MODIFIER_COUNT;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::monsters::Intent;
use crate::monsters::Monster;
use crate::monsters::Move;
use crate::state::Vitals;
use crate::types::MonsterKind;
use crate::types::EntityId;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_CHOMP_11: Move = Move {
    name: "Chomp",
    effects: &[EffectTemplate::DamagePhysical {
        base: 11,
        target: TargetKind::Character,
    }],
    intent: Intent::Attack {
        damage: 11,
        instances: 1,
    },
};
static MOVE_CHOMP_12: Move = Move {
    name: "Chomp",
    effects: &[EffectTemplate::DamagePhysical {
        base: 12,
        target: TargetKind::Character,
    }],
    intent: Intent::Attack {
        damage: 12,
        instances: 1,
    },
};
static MOVE_THRASH: Move = Move {
    name: "Thrash",
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 7,
            target: TargetKind::Character,
        },
        EffectTemplate::BlockGain {
            amount: 5,
            target: TargetKind::Source,
        },
    ],
    intent: Intent::AttackBlock {
        damage: 7,
        instances: 1,
    },
};
static MOVE_BELLOW_3_6: Move = Move {
    name: "Bellow",
    effects: &[
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 3,
            target: TargetKind::Source,
        },
        EffectTemplate::BlockGain {
            amount: 6,
            target: TargetKind::Source,
        },
    ],
    intent: Intent::BlockBuff,
};
static MOVE_BELLOW_4_6: Move = Move {
    name: "Bellow",
    effects: &[
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 4, // +1 strength
            target: TargetKind::Source,
        },
        EffectTemplate::BlockGain {
            amount: 6,
            target: TargetKind::Source,
        },
    ],
    intent: Intent::BlockBuff,
};
static MOVE_BELLOW_5_9: Move = Move {
    name: "Bellow",
    effects: &[
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 5, // +2 strength
            target: TargetKind::Source,
        },
        EffectTemplate::BlockGain {
            amount: 9, // +3 damage
            target: TargetKind::Source,
        },
    ],
    intent: Intent::BlockBuff,
};
static MOVES_ASC0: [Move; 3] = [MOVE_CHOMP_11, MOVE_BELLOW_3_6, MOVE_THRASH];
static MOVES_ASC2: [Move; 3] = [MOVE_CHOMP_12, MOVE_BELLOW_4_6, MOVE_THRASH];
static MOVES_ASC17: [Move; 3] = [MOVE_CHOMP_12, MOVE_BELLOW_5_9, MOVE_THRASH];

pub fn spawn_jaw_worm(id: EntityId, ascension_level: u8, rng: &mut impl Rng) -> Monster {
    // Roll max health
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (40, 44)
    } else {
        (42, 46)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    // Moves
    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    Monster {
        id,
        name: MonsterName::JawWorm,
        kind: MonsterKind::Normal,
        vitals: Vitals {
            health: health_max,
            health_max: health_max,
            block: 0,
            modifiers: Modifiers {
                stacks: [0; MODIFIER_COUNT],
                is_new: [false; MODIFIER_COUNT],
                active: 0,
            },
        },
        moves: moves,
        move_current: None,
        move_history: Vec::new(),
    }
}

pub fn get_next_move_jaw_worm(monster: &Monster, rng: &mut impl Rng) -> usize {
    if monster.move_current.is_none() {
        return 0; // Chomp
    }

    let roll = rng.random_range(0..99);
    let move_last = monster
        .move_history
        .last()
        .copied()
        .expect("`move_history` cannot be empty here");

    if roll < 25 {
        if move_last == 0 {
            return if rng.random_bool(0.5625) { 1 } else { 2 };
        }
        0 // Chomp
    } else if roll < 55 {
        if monster.move_history.ends_with(&[2, 2]) {
            // two Thrashes
            return if rng.random_bool(0.357) { 0 } else { 1 };
        }
        2 // Thrash
    } else {
        if move_last == 1 {
            return if rng.random_bool(0.416) { 0 } else { 2 };
        }
        1 // Bellow
    }
}
