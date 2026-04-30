use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS, modifier_apply, modifier_has};
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

// Lagavulin (Elite). Spawns asleep with Metallicize 8 + 8 starting block.
//
// Sleep flow (no `monster_param`, derived from `move_history`):
//   - while Asleep: trailing-Sleep count < 2 → Sleep; == 2 → SleepFinal.
//   - SleepFinal's effects = [RemoveAsleep, RemoveMetallicize], so the
//     T3 Metallicize tick (which fires after move effects in
//     `process_effect_turn_end_monster`) sees Metallicize already gone.
//     Matches Java's mid-T3 ChangeStateAction OPEN timing.
//   - On HP loss while Asleep: the on-damage hook in `process_effect_damage_deal`
//     sets move_current = Stunned and queues the same removals.
//
// Awake rotation: Attack → Attack → Siphon Soul → Attack → Attack → Siphon
// → ... Derived from move_history: count of trailing Attack+Stunned+SleepFinal
// entries that are NOT Siphon — when 2 trailing Attacks (or any wake transition
// followed by Attacks) since last Siphon → fire Siphon. Stunned and SleepFinal
// transition into the rotation as "fresh wake → next is Attack".

static MOVE_SLEEP: Move = Move {
    name: "Sleep",
    effects: &[],
    intent: Intent::Sleep,
};
static MOVE_SLEEP_FINAL: Move = Move {
    name: "Sleep",
    effects: &[
        Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Asleep,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Metallicize,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::Sleep,
};
static MOVE_STUNNED: Move = Move {
    name: "Stunned",
    effects: &[],
    intent: Intent::Sleep,
};
static MOVE_ATTACK_18: Move = Move {
    name: "Attack",
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
static MOVE_ATTACK_20: Move = Move {
    name: "Attack",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 20 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 20,
        instances: 1,
    },
};
static MOVE_SIPHON_SOUL_1: Move = Move {
    name: "Siphon Soul",
    effects: &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: -1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Dexterity,
                stacks: -1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::DebuffPowerful,
};
static MOVE_SIPHON_SOUL_2: Move = Move {
    name: "Siphon Soul",
    effects: &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: -2,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Dexterity,
                stacks: -2,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::DebuffPowerful,
};

static MOVES_ASC0: [Move; 5] = [
    MOVE_SLEEP,
    MOVE_SLEEP_FINAL,
    MOVE_STUNNED,
    MOVE_ATTACK_18,
    MOVE_SIPHON_SOUL_1,
];
static MOVES_ASC3: [Move; 5] = [
    MOVE_SLEEP,
    MOVE_SLEEP_FINAL,
    MOVE_STUNNED,
    MOVE_ATTACK_20,
    MOVE_SIPHON_SOUL_1,
];
static MOVES_ASC18: [Move; 5] = [
    MOVE_SLEEP,
    MOVE_SLEEP_FINAL,
    MOVE_STUNNED,
    MOVE_ATTACK_20,
    MOVE_SIPHON_SOUL_2,
];

const IDX_MOVE_SLEEP: usize = 0;
const IDX_MOVE_SLEEP_FINAL: usize = 1;
pub const IDX_MOVE_STUNNED: usize = 2;
const IDX_MOVE_ATTACK: usize = 3;
const IDX_MOVE_SIPHON: usize = 4;

pub fn spawn_lagavulin(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 8 {
        (109, 111)
    } else {
        (112, 115)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 3 {
        &MOVES_ASC0
    } else if ascension_level < 18 {
        &MOVES_ASC3
    } else {
        &MOVES_ASC18
    };

    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Asleep, 1);
    modifier_apply(&mut modifiers, ModifierKind::Metallicize, 8);

    make_entity_monster(
        MonsterName::Lagavulin,
        MonsterKind::Elite,
        Vitals {
            health: health_max,
            health_max,
            block: 8,
        },
        modifiers,
        moves,
    )
}

pub fn get_next_move_lagavulin(
    move_current: Option<usize>,
    move_history: &[u8],
    modifiers: &crate::modifier::Modifiers,
) -> usize {
    // Combat start: no move yet → Sleep (Asleep is innate).
    if move_current.is_none() {
        return IDX_MOVE_SLEEP;
    }

    if modifier_has(modifiers, ModifierKind::Asleep) {
        // Count trailing Sleep moves in history.
        let trailing_sleeps = move_history
            .iter()
            .rev()
            .take_while(|&&m| m == IDX_MOVE_SLEEP as u8)
            .count();
        if trailing_sleeps < 2 {
            IDX_MOVE_SLEEP
        } else {
            IDX_MOVE_SLEEP_FINAL
        }
    } else {
        // Awake rotation: 2 Attacks then Siphon Soul.
        // Count trailing Attack moves in history (after the most recent
        // wake/Siphon). If >= 2 → Siphon. Else → Attack.
        let trailing_attacks = move_history
            .iter()
            .rev()
            .take_while(|&&m| m == IDX_MOVE_ATTACK as u8)
            .count();
        if trailing_attacks >= 2 {
            IDX_MOVE_SIPHON
        } else {
            IDX_MOVE_ATTACK
        }
    }
}
