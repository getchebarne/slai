use crate::effect::EffectTemplate;
use crate::effect::TargetKind;
use crate::modifier::ModifierKind;
use crate::modifier::modifiers_new;
use crate::monsters::Intent;
use crate::monsters::Move;
use crate::monsters::Monster;
use crate::monsters::MAX_MOVE_HISTORY;
use crate::state::Vitals;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_CHOMP_11: Move = Move {
    name: "Chomp",
    effects: &[EffectTemplate::DamagePhysical {
        base: 11,
        target: TargetKind::Character,
    }],
    intent: Intent::Attack { damage: 11, instances: 1 },
};
static MOVE_CHOMP_12: Move = Move {
    name: "Chomp",
    effects: &[EffectTemplate::DamagePhysical {
        base: 12,
        target: TargetKind::Character,
    }],
    intent: Intent::Attack { damage: 12, instances: 1 },
};
static MOVE_THRASH: Move = Move {
    name: "Thrash",
    effects: &[
        EffectTemplate::DamagePhysical { base: 7, target: TargetKind::Character },
        EffectTemplate::BlockGain { amount: 5, target: TargetKind::Source },
    ],
    intent: Intent::AttackBlock { damage: 7, instances: 1 },
};
static MOVE_BELLOW_3_6: Move = Move {
    name: "Bellow",
    effects: &[
        EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 3, target: TargetKind::Source },
        EffectTemplate::BlockGain { amount: 6, target: TargetKind::Source },
    ],
    intent: Intent::BlockBuff,
};
static MOVE_BELLOW_4_6: Move = Move {
    name: "Bellow",
    effects: &[
        EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 4, target: TargetKind::Source },
        EffectTemplate::BlockGain { amount: 6, target: TargetKind::Source },
    ],
    intent: Intent::BlockBuff,
};
static MOVE_BELLOW_5_9: Move = Move {
    name: "Bellow",
    effects: &[
        EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 5, target: TargetKind::Source },
        EffectTemplate::BlockGain { amount: 9, target: TargetKind::Source },
    ],
    intent: Intent::BlockBuff,
};
static MOVES_ASC0: [Move; 3] = [MOVE_CHOMP_11, MOVE_BELLOW_3_6, MOVE_THRASH];
static MOVES_ASC2: [Move; 3] = [MOVE_CHOMP_12, MOVE_BELLOW_4_6, MOVE_THRASH];
static MOVES_ASC17: [Move; 3] = [MOVE_CHOMP_12, MOVE_BELLOW_5_9, MOVE_THRASH];

pub fn spawn_jaw_worm(ascension_level: u8, rng: &mut impl Rng) -> Monster {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (40, 44)
    } else {
        (42, 46)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    Monster {
        name: MonsterName::JawWorm,
        monster_kind: MonsterKind::Normal,
        vitals: Vitals { health: health_max, health_max, block: 0 },
        modifiers: modifiers_new(),
        moves,
        move_current: None,
        move_history: [0; MAX_MOVE_HISTORY],
        move_history_len: 0,
    }
}

pub fn get_next_move_jaw_worm(
    move_current: Option<usize>,
    move_history: &[u8],
    _moves: &[Move],
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return 0;
    }

    let roll = rng.random_range(0..99);
    let move_last = move_history
        .last()
        .copied()
        .expect("`move_history` cannot be empty here");

    if roll < 25 {
        if move_last == 0 {
            return if rng.random_bool(0.5625) { 1 } else { 2 };
        }
        0
    } else if roll < 55 {
        if move_history.ends_with(&[2, 2]) {
            return if rng.random_bool(0.357) { 0 } else { 1 };
        }
        2
    } else {
        if move_last == 1 {
            return if rng.random_bool(0.416) { 0 } else { 2 };
        }
        1
    }
}
