use crate::consts::MAX_MONSTERS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_BEAM_9: Move = move_attack("Beam", 9, 1);
static MOVE_BEAM_10: Move = move_attack("Beam", 10, 1);
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

pub static SENTRY: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Sentry,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (38, 42)), (8, (39, 45))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (3, &MOVES_ASC3), (18, &MOVES_ASC18)],
    modifier_tiers: &[(0, &[(ModifierKind::Artifact, 1)])],
};

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
