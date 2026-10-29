use crate::consts::MAX_MONSTERS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ScaledAt;
use crate::effect::Summon;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// The four dagger posts' slots, in fill order, around Reptomancer's slot 2; the opening daggers hold the first two
const DAGGER_POSTS: [usize; 4] = [4, 1, 3, 0];

// One dagger into the first free post, as a Minion
static SUMMON_DAGGER: Summon = Summon {
    pool: &[MonsterName::SnakeDagger],
    posts: &DAGGER_POSTS,
    minion: true,
};

// Summon's one effect per dagger
const EFFECT_DAGGER_SUMMON: Effect = Effect {
    kind: EffectKind::MonsterSummon {
        summon: &SUMMON_DAGGER,
    },
    id_source: None,
    target: Target::Direct(None),
};

// Snake Strike: two hits, then Weak 1
const fn move_snake_strike(damage: u16) -> Move {
    make_move(
        "Snake Strike",
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    instances: 2,
                    lifesteal: false,
                    if_poisoned: false,
                    scaled_at: ScaledAt::Now,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierDelta {
                    kind: ModifierKind::Weak,
                    stacks: 1,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
        ],
        Intent::AttackDebuff {
            damage,
            instances: 2,
        },
    )
}

static MOVE_SNAKE_STRIKE_13: Move = move_snake_strike(13);
static MOVE_SNAKE_STRIKE_16: Move = move_snake_strike(16);
// Summon: one dagger, two from A18
static MOVE_SUMMON_1: Move = make_move("Summon", &[EFFECT_DAGGER_SUMMON], Intent::Unknown);
static MOVE_SUMMON_2: Move = make_move(
    "Summon",
    &[EFFECT_DAGGER_SUMMON, EFFECT_DAGGER_SUMMON],
    Intent::Unknown,
);
static MOVE_BIG_BITE_30: Move = move_attack("Big Bite", 30, 1);
static MOVE_BIG_BITE_34: Move = move_attack("Big Bite", 34, 1);

static MOVES_ASC0: [Move; 3] = [MOVE_SNAKE_STRIKE_13, MOVE_SUMMON_1, MOVE_BIG_BITE_30];
static MOVES_ASC3: [Move; 3] = [MOVE_SNAKE_STRIKE_16, MOVE_SUMMON_1, MOVE_BIG_BITE_34];
static MOVES_ASC18: [Move; 3] = [MOVE_SNAKE_STRIKE_16, MOVE_SUMMON_2, MOVE_BIG_BITE_34];

const IDX_MOVE_SNAKE_STRIKE: usize = 0;
const IDX_MOVE_SUMMON: usize = 1;
const IDX_MOVE_BIG_BITE: usize = 2;

pub static REPTOMANCER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Reptomancer,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (180, 190)), (8, (190, 200))],
    block_tiers: &[],
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (3, &[&MOVES_ASC3]),
        (18, &[&MOVES_ASC18]),
    ],
    modifier_tiers: &[],
};

// Summon first; after it one roll per move, and Snake Strike's or Big Bite's band rolls once more over the other two bands when its move just came
pub fn get_next_move_reptomancer(
    move_history: &[u8],
    entity_id: usize,
    id_monsters: &[Option<usize>; MAX_MONSTERS],
    rng: &mut impl Rng,
) -> usize {
    if move_history.is_empty() {
        return IDX_MOVE_SUMMON;
    }

    let daggers_alive = id_monsters
        .iter()
        .flatten()
        .filter(|&&id| id != entity_id)
        .count();
    let move_last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    let roll = rng.random_range(0..=99);
    let roll = if roll < 33 && move_last == Some(IDX_MOVE_SNAKE_STRIKE) {
        rng.random_range(33..=99)
    } else if roll >= 66 && move_last == Some(IDX_MOVE_BIG_BITE) {
        rng.random_range(0..=65)
    } else {
        roll
    };

    if roll < 33 {
        IDX_MOVE_SNAKE_STRIKE
    } else if roll < 66 {
        // Summon needs at most three other Monsters standing, and never comes a third time in a row
        if daggers_alive <= 3
            && !move_history.ends_with(&[IDX_MOVE_SUMMON as u8, IDX_MOVE_SUMMON as u8])
        {
            IDX_MOVE_SUMMON
        } else {
            IDX_MOVE_SNAKE_STRIKE
        }
    } else {
        IDX_MOVE_BIG_BITE
    }
}
