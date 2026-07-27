use crate::consts::MAX_MONSTERS;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::entity::make_entity_monster;
use crate::entity::make_move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_BEAM_9: Move = make_move(
    "Beam",
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 9 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    Intent::Attack {
        damage: 9,
        instances: 1,
    },
);
static MOVE_BEAM_10: Move = make_move(
    "Beam",
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 10 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    Intent::Attack {
        damage: 10,
        instances: 1,
    },
);
static MOVE_BOLT_2: Move = make_move(
    "Bolt",
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Dazed,
            pile: CardPile::Discard,
            count: 2,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    Intent::Debuff,
);
static MOVE_BOLT_3: Move = make_move(
    "Bolt",
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Dazed,
            pile: CardPile::Discard,
            count: 3,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    Intent::Debuff,
);

static MOVES_ASC0: [Move; 2] = [MOVE_BEAM_9, MOVE_BOLT_2];
static MOVES_ASC3: [Move; 2] = [MOVE_BEAM_10, MOVE_BOLT_2];
static MOVES_ASC18: [Move; 2] = [MOVE_BEAM_10, MOVE_BOLT_3];

const IDX_MOVE_BEAM: usize = 0;
const IDX_MOVE_BOLT: usize = 1;

pub fn spawn_monster_sentry(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 8 {
        (38, 42)
    } else {
        (39, 45)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 3 {
        &MOVES_ASC0
    } else if ascension_level < 18 {
        &MOVES_ASC3
    } else {
        &MOVES_ASC18
    };

    // Spawn with 1 Artifact stack
    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Artifact, 1);

    make_entity_monster(
        MonsterName::Sentry,
        MonsterKind::Elite,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        moves,
    )
}

// Strict Bolt↔Beam alternation; first is Bolt at even roster index, Beam at odd
pub fn get_next_move_sentry(
    move_current: Option<usize>,
    move_history: &[u8],
    entity_id: usize,
    id_monsters: &[Option<usize>; MAX_MONSTERS],
) -> usize {
    if move_current.is_none() {
        let position = id_monsters
            .iter()
            .flatten()
            .position(|&id| id == entity_id)
            .unwrap_or(0);
        return if position % 2 == 0 {
            IDX_MOVE_BOLT
        } else {
            IDX_MOVE_BEAM
        };
    }
    // Subsequent moves: strict alternation
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;
    if last == IDX_MOVE_BEAM {
        IDX_MOVE_BOLT
    } else {
        IDX_MOVE_BEAM
    }
}
