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
use crate::types::MonsterName;
use rand::Rng;

static MOVE_DARK_STRIKE: Move = Move {
    name: "Dark Strike",
    effects: &[EffectTemplate::DamagePhysical {
        base: 6,
        target: TargetKind::Character,
    }],
    intent: Intent::Attack {
        damage: 6,
        instances: 1,
    },
};
static MOVE_INCANTATION_3: Move = Move {
    name: "Incantation",
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Ritual,
        stacks: 3,
        target: TargetKind::Source,
    }],
    intent: Intent::Buff,
};
static MOVE_INCANTATION_4: Move = Move {
    name: "Incantation",
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Ritual,
        stacks: 4, // +1 ritual
        target: TargetKind::Source,
    }],
    intent: Intent::Buff,
};
static MOVE_INCANTATION_5: Move = Move {
    name: "Incantation",
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Ritual,
        stacks: 5, // +2 ritual
        target: TargetKind::Source,
    }],
    intent: Intent::Buff,
};
static MOVES_ASC0: [Move; 2] = [MOVE_INCANTATION_3, MOVE_DARK_STRIKE];
static MOVES_ASC2: [Move; 2] = [MOVE_INCANTATION_4, MOVE_DARK_STRIKE];
static MOVES_ASC17: [Move; 2] = [MOVE_INCANTATION_5, MOVE_DARK_STRIKE];

pub fn spawn_cultist(ascension_level: u8, rng: &mut impl Rng) -> Monster {
    // Roll max health
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (48, 54)
    } else {
        (50, 56)
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
        name: MonsterName::Cultist,
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

pub fn get_next_move_cultist(monster: &Monster) -> usize {
    if monster.move_current.is_none() {
        0 // Incantation
    } else {
        1 // Dark strike
    }
}
