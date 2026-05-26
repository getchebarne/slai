use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::entity::make_entity_monster;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::types::CardName;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;

static MOVE_GOOP_SPRAY_3: Move = Move {
    name: "Goop Spray",
    effects: &[Effect {
        kind: EffectKind::CardAddToDiscard {
            card_name: CardName::Slimed,
            count: 3,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    intent: Intent::DebuffPowerful,
};
static MOVE_GOOP_SPRAY_5: Move = Move {
    name: "Goop Spray",
    effects: &[Effect {
        kind: EffectKind::CardAddToDiscard {
            card_name: CardName::Slimed,
            count: 5,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    intent: Intent::DebuffPowerful,
};
static MOVE_PREPARING: Move = Move {
    name: "Preparing",
    effects: &[],
    intent: Intent::Unknown,
};
static MOVE_SLAM_35: Move = Move {
    name: "Slam",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 35 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 35,
        instances: 1,
    },
};
static MOVE_SLAM_38: Move = Move {
    name: "Slam",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 38 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 38,
        instances: 1,
    },
};
static MOVE_SPLIT: Move = Move {
    name: "Split",
    effects: &[
        Effect {
            kind: EffectKind::MonsterSpawn {
                name: MonsterName::SlimeSpikeLarge,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Source,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::MonsterSpawn {
                name: MonsterName::SlimeAcidLarge,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Source,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::EscapeMonster,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Source,
                selection_kind: SelectionKind::Single,
            },
        },
    ],
    intent: Intent::Unknown,
};

static MOVES_ASC0: [Move; 4] = [MOVE_GOOP_SPRAY_3, MOVE_PREPARING, MOVE_SLAM_35, MOVE_SPLIT];
static MOVES_ASC4: [Move; 4] = [MOVE_GOOP_SPRAY_3, MOVE_PREPARING, MOVE_SLAM_38, MOVE_SPLIT];
static MOVES_ASC19: [Move; 4] = [MOVE_GOOP_SPRAY_5, MOVE_PREPARING, MOVE_SLAM_38, MOVE_SPLIT];

const IDX_MOVE_GOOP_SPRAY: usize = 0;
const IDX_MOVE_PREPARING: usize = 1;
const IDX_MOVE_SLAM: usize = 2;
pub const IDX_MOVE_SPLIT: usize = 3;

pub fn spawn_monster_slime_boss(ascension_level: u8) -> Entity {
    let health_max: u16 = if ascension_level < 9 { 140 } else { 150 };

    let moves: &'static [Move] = if ascension_level < 4 {
        &MOVES_ASC0
    } else if ascension_level < 19 {
        &MOVES_ASC4
    } else {
        &MOVES_ASC19
    };

    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Splittable, 1);

    make_entity_monster(
        MonsterName::SlimeBoss,
        MonsterKind::Boss,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        moves,
    )
}

pub fn get_next_move_slime_boss(move_current: Option<usize>, move_history: &[u8]) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_GOOP_SPRAY;
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;
    match last {
        IDX_MOVE_GOOP_SPRAY => IDX_MOVE_PREPARING,
        IDX_MOVE_PREPARING => IDX_MOVE_SLAM,
        IDX_MOVE_SLAM => IDX_MOVE_GOOP_SPRAY,
        IDX_MOVE_SPLIT => IDX_MOVE_SPLIT,
        _ => unreachable!("SlimeBoss unexpected move idx: {last}"),
    }
}
