use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS};
use crate::types::{CardName, MonsterKind, MonsterName, Vitals};
use rand::Rng;

static MOVE_WOUND_TACKLE_7: Move = Move {
    name: "Corrosive Spit",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 7,
                condition: DamageCondition::Always,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::CardAddToDiscard {
                card_name: CardName::Slimed,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 7,
        instances: 1,
    },
};
static MOVE_WOUND_TACKLE_8: Move = Move {
    name: "Corrosive Spit",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 8,
                condition: DamageCondition::Always,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::CardAddToDiscard {
                card_name: CardName::Slimed,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 8,
        instances: 1,
    },
};
static MOVE_HEAVY_TACKLE_10: Move = Move {
    name: "Tackle",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 10,
            condition: DamageCondition::Always,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 10,
        instances: 1,
    },
};
static MOVE_HEAVY_TACKLE_12: Move = Move {
    name: "Tackle",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 12,
            condition: DamageCondition::Always,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 12,
        instances: 1,
    },
};
static MOVE_LICK: Move = Move {
    name: "Lick",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    intent: Intent::Debuff,
};

static MOVES_ASC0: [Move; 3] = [MOVE_WOUND_TACKLE_7, MOVE_HEAVY_TACKLE_10, MOVE_LICK];
static MOVES_ASC2: [Move; 3] = [MOVE_WOUND_TACKLE_8, MOVE_HEAVY_TACKLE_12, MOVE_LICK];

const IDX_MOVE_WOUND_TACKLE: usize = 0;
const IDX_MOVE_HEAVY_TACKLE: usize = 1;
const IDX_MOVE_LICK: usize = 2;

pub fn spawn_slime_acid_medium(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (28, 32)
    } else {
        (29, 34)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else {
        &MOVES_ASC2
    };

    make_entity_monster(
        MonsterName::SlimeAcidMedium,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        ZERO_MODIFIERS,
        moves,
    )
}

pub fn get_next_move_slime_acid_medium(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    if ascension_level >= 17 {
        // Asc 17+: 40/40/20 split with stricter constraints
        if roll < 40 {
            // Prefer Wound Tackle, but not three in a row
            if move_history.ends_with(&[IDX_MOVE_WOUND_TACKLE as u8, IDX_MOVE_WOUND_TACKLE as u8]) {
                if rng.random_bool(0.5) {
                    IDX_MOVE_HEAVY_TACKLE
                } else {
                    IDX_MOVE_LICK
                }
            } else {
                IDX_MOVE_WOUND_TACKLE
            }
        } else if roll < 80 {
            // Prefer Heavy Tackle, but not three in a row
            if move_history.ends_with(&[IDX_MOVE_HEAVY_TACKLE as u8, IDX_MOVE_HEAVY_TACKLE as u8]) {
                if rng.random_bool(0.5) {
                    IDX_MOVE_WOUND_TACKLE
                } else {
                    IDX_MOVE_LICK
                }
            } else {
                IDX_MOVE_HEAVY_TACKLE
            }
        } else if move_history.last().copied() == Some(IDX_MOVE_LICK as u8) {
            // Lick: Asc 17+ no-two-in-a-row
            if rng.random_bool(0.4) {
                IDX_MOVE_WOUND_TACKLE
            } else {
                IDX_MOVE_HEAVY_TACKLE
            }
        } else {
            IDX_MOVE_LICK
        }
    } else if roll < 30 {
        // Wound Tackle: Asc 0-16 no-three-in-a-row
        if move_history.ends_with(&[IDX_MOVE_WOUND_TACKLE as u8, IDX_MOVE_WOUND_TACKLE as u8]) {
            if rng.random_bool(0.5) {
                IDX_MOVE_HEAVY_TACKLE
            } else {
                IDX_MOVE_LICK
            }
        } else {
            IDX_MOVE_WOUND_TACKLE
        }
    } else if roll < 70 {
        // Heavy Tackle: Asc 0-16 no-two-in-a-row (looser than Asc 17+)
        if move_history.last().copied() == Some(IDX_MOVE_HEAVY_TACKLE as u8) {
            if rng.random_bool(0.4) {
                IDX_MOVE_WOUND_TACKLE
            } else {
                IDX_MOVE_LICK
            }
        } else {
            IDX_MOVE_HEAVY_TACKLE
        }
    } else if move_history.ends_with(&[IDX_MOVE_LICK as u8, IDX_MOVE_LICK as u8]) {
        // Lick: Asc 0-16 no-three-in-a-row.
        if rng.random_bool(0.4) {
            IDX_MOVE_WOUND_TACKLE
        } else {
            IDX_MOVE_HEAVY_TACKLE
        }
    } else {
        IDX_MOVE_LICK
    }
}
