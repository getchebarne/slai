use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS, modifier_apply};
use crate::types::{CardName, MonsterKind, MonsterName, Vitals};
use rand::Rng;

static MOVE_FLAME_TACKLE_16: Move = Move {
    name: "Flame Tackle",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 16 },
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
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 16,
        instances: 1,
    },
};
static MOVE_FLAME_TACKLE_18: Move = Move {
    name: "Flame Tackle",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 18 },
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
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 18,
        instances: 1,
    },
};
static MOVE_LICK_FRAIL_2: Move = Move {
    name: "Lick",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Frail,
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
static MOVE_LICK_FRAIL_3: Move = Move {
    name: "Lick",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Frail,
            stacks: 3,
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
                name: MonsterName::SlimeSpikeMedium,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::SpawnMonster {
                name: MonsterName::SlimeSpikeMedium,
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

static MOVES_ASC0: [Move; 3] = [MOVE_FLAME_TACKLE_16, MOVE_LICK_FRAIL_2, MOVE_SPLIT];
static MOVES_ASC2: [Move; 3] = [MOVE_FLAME_TACKLE_18, MOVE_LICK_FRAIL_2, MOVE_SPLIT];
static MOVES_ASC17: [Move; 3] = [MOVE_FLAME_TACKLE_18, MOVE_LICK_FRAIL_3, MOVE_SPLIT];

const IDX_MOVE_FLAME_TACKLE: usize = 0;
const IDX_MOVE_LICK: usize = 1;
pub const IDX_MOVE_SPLIT: usize = 2;

pub fn spawn_slime_spike_large(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (64, 70)
    } else {
        (67, 73)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Splittable, 1);

    make_entity_monster(
        MonsterName::SlimeSpikeLarge,
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

pub fn get_next_move_slime_spike_large(
    _move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    if ascension_level >= 17 {
        if roll < 30 {
            // Flame Tackle: Asc 17+ no-two-in-a-row → fall back to Lick.
            if move_history.ends_with(&[IDX_MOVE_FLAME_TACKLE as u8, IDX_MOVE_FLAME_TACKLE as u8]) {
                IDX_MOVE_LICK
            } else {
                IDX_MOVE_FLAME_TACKLE
            }
        } else if move_history.last().copied() == Some(IDX_MOVE_LICK as u8) {
            // Lick: Asc 17+ no-two-in-a-row → fall back to Flame Tackle.
            IDX_MOVE_FLAME_TACKLE
        } else {
            IDX_MOVE_LICK
        }
    } else if roll < 30 {
        // Flame Tackle: Asc 0-16 no-three-in-a-row → fall back to Lick.
        if move_history.ends_with(&[IDX_MOVE_FLAME_TACKLE as u8, IDX_MOVE_FLAME_TACKLE as u8]) {
            IDX_MOVE_LICK
        } else {
            IDX_MOVE_FLAME_TACKLE
        }
    } else if move_history.ends_with(&[IDX_MOVE_LICK as u8, IDX_MOVE_LICK as u8]) {
        // Lick: Asc 0-16 no-three-in-a-row → fall back to Flame Tackle.
        IDX_MOVE_FLAME_TACKLE
    } else {
        IDX_MOVE_LICK
    }
}
