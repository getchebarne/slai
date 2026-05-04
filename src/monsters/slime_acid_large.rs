use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS, modifier_apply};
use crate::types::{CardName, MonsterKind, MonsterName, Vitals};
use rand::Rng;

static MOVE_SLIME_TACKLE_11: Move = Move {
    name: "Corrosive Spit",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 11 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardAddToDiscard {
                card_name: CardName::Slimed,
                count: 2,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 11,
        instances: 1,
    },
};
static MOVE_SLIME_TACKLE_12: Move = Move {
    name: "Corrosive Spit",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 12 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::CardAddToDiscard {
                card_name: CardName::Slimed,
                count: 2,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 12,
        instances: 1,
    },
};
static MOVE_HEAVY_TACKLE_16: Move = Move {
    name: "Tackle",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 16 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 16,
        instances: 1,
    },
};
static MOVE_HEAVY_TACKLE_18: Move = Move {
    name: "Tackle",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 18 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 18,
        instances: 1,
    },
};
static MOVE_LICK: Move = Move {
    name: "Lick",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 2,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Debuff,
};
static MOVE_SPLIT: Move = Move {
    name: "Split",
    effects: &[
        Effect {
            kind: EffectKind::SpawnMonster {
                name: MonsterName::SlimeAcidMedium,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::SpawnMonster {
                name: MonsterName::SlimeAcidMedium,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::EscapeMonster,
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::Unknown,
};

static MOVES_ASC0: [Move; 4] = [
    MOVE_SLIME_TACKLE_11,
    MOVE_HEAVY_TACKLE_16,
    MOVE_LICK,
    MOVE_SPLIT,
];
static MOVES_ASC2: [Move; 4] = [
    MOVE_SLIME_TACKLE_12,
    MOVE_HEAVY_TACKLE_18,
    MOVE_LICK,
    MOVE_SPLIT,
];

const IDX_MOVE_SLIME_TACKLE: usize = 0;
const IDX_MOVE_HEAVY_TACKLE: usize = 1;
const IDX_MOVE_LICK: usize = 2;
pub const IDX_MOVE_SPLIT: usize = 3;

pub fn spawn_slime_acid_large(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (65, 69)
    } else {
        (68, 72)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else {
        &MOVES_ASC2
    };

    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Splittable, 1);

    make_entity_monster(
        MonsterName::SlimeAcidLarge,
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

pub fn get_next_move_slime_acid_large(
    _move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    if ascension_level >= 17 {
        // Asc 17+: 40/30/30 split. Slime Tackle no-three-in-a-row,
        // Heavy no-three-in-a-row, Lick no-two-in-a-row
        if roll < 40 {
            if move_history.ends_with(&[IDX_MOVE_SLIME_TACKLE as u8, IDX_MOVE_SLIME_TACKLE as u8]) {
                if rng.random_bool(0.6) {
                    IDX_MOVE_HEAVY_TACKLE
                } else {
                    IDX_MOVE_LICK
                }
            } else {
                IDX_MOVE_SLIME_TACKLE
            }
        } else if roll < 70 {
            if move_history.ends_with(&[IDX_MOVE_HEAVY_TACKLE as u8, IDX_MOVE_HEAVY_TACKLE as u8]) {
                if rng.random_bool(0.6) {
                    IDX_MOVE_SLIME_TACKLE
                } else {
                    IDX_MOVE_LICK
                }
            } else {
                IDX_MOVE_HEAVY_TACKLE
            }
        } else if move_history.last().copied() == Some(IDX_MOVE_LICK as u8) {
            if rng.random_bool(0.4) {
                IDX_MOVE_SLIME_TACKLE
            } else {
                IDX_MOVE_HEAVY_TACKLE
            }
        } else {
            IDX_MOVE_LICK
        }
    } else if roll < 30 {
        // Asc 0-16: 30/40/30 split. Slime Tackle no-three-in-a-row,
        // Heavy no-two-in-a-row, Lick no-three-in-a-row
        if move_history.ends_with(&[IDX_MOVE_SLIME_TACKLE as u8, IDX_MOVE_SLIME_TACKLE as u8]) {
            if rng.random_bool(0.5) {
                IDX_MOVE_HEAVY_TACKLE
            } else {
                IDX_MOVE_LICK
            }
        } else {
            IDX_MOVE_SLIME_TACKLE
        }
    } else if roll < 70 {
        if move_history.last().copied() == Some(IDX_MOVE_HEAVY_TACKLE as u8) {
            if rng.random_bool(0.4) {
                IDX_MOVE_SLIME_TACKLE
            } else {
                IDX_MOVE_LICK
            }
        } else {
            IDX_MOVE_HEAVY_TACKLE
        }
    } else if move_history.ends_with(&[IDX_MOVE_LICK as u8, IDX_MOVE_LICK as u8]) {
        if rng.random_bool(0.4) {
            IDX_MOVE_SLIME_TACKLE
        } else {
            IDX_MOVE_HEAVY_TACKLE
        }
    } else {
        IDX_MOVE_LICK
    }
}
