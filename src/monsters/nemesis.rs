use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// Every move ends with Intangible 1 on Nemesis, unless it already holds Intangible
const EFFECT_INTANGIBLE_GAIN: Effect = Effect {
    kind: EffectKind::ModifierDelta {
        kind: ModifierKind::Intangible,
        stacks: 1,
    },
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Source,
        filters: &[CandidateFilter::NotIntangible],
        selection_kind: SelectionKind::All,
    },
};

const fn move_attack_intangible(name: &'static str, damage: u16, instances: u8) -> Move {
    make_move(
        name,
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    instances: instances as u16,
                    lifesteal: false,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            EFFECT_INTANGIBLE_GAIN,
        ],
        Intent::Attack { damage, instances },
    )
}

const fn move_tri_burn(count: u16) -> Move {
    make_move(
        "Tri Burn",
        &[
            Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::Burn,
                    card_pile: CardPile::Discard,
                    count,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            },
            EFFECT_INTANGIBLE_GAIN,
        ],
        Intent::Debuff,
    )
}

static MOVE_TRI_ATTACK_6: Move = move_attack_intangible("Tri Attack", 6, 3);
static MOVE_TRI_ATTACK_7: Move = move_attack_intangible("Tri Attack", 7, 3);
static MOVE_SCYTHE: Move = move_attack_intangible("Scythe", 45, 1);
static MOVE_TRI_BURN_3: Move = move_tri_burn(3);
static MOVE_TRI_BURN_5: Move = move_tri_burn(5);

static MOVES_ASC0: [Move; 3] = [MOVE_TRI_ATTACK_6, MOVE_SCYTHE, MOVE_TRI_BURN_3];
static MOVES_ASC3: [Move; 3] = [MOVE_TRI_ATTACK_7, MOVE_SCYTHE, MOVE_TRI_BURN_3];
static MOVES_ASC18: [Move; 3] = [MOVE_TRI_ATTACK_7, MOVE_SCYTHE, MOVE_TRI_BURN_5];

const IDX_MOVE_TRI_ATTACK: usize = 0;
const IDX_MOVE_SCYTHE: usize = 1;
const IDX_MOVE_TRI_BURN: usize = 2;

pub static NEMESIS: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Nemesis,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (185, 185)), (8, (200, 200))],
    block_tiers: &[],
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (3, &[&MOVES_ASC3]),
        (18, &[&MOVES_ASC18]),
    ],
    modifier_tiers: &[],
};

// One roll per move; when the history rules out its band's move, one coin picks between the other two
pub fn get_next_move_nemesis(
    move_current: Option<usize>,
    move_history: &[u8],
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);

    // The opening move is never Scythe
    if move_current.is_none() {
        return if roll < 50 {
            IDX_MOVE_TRI_ATTACK
        } else {
            IDX_MOVE_TRI_BURN
        };
    }

    // Scythe and Tri Burn never come twice in a row, Tri Attack never three times
    let last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    if roll < 30 {
        if last != Some(IDX_MOVE_SCYTHE) {
            IDX_MOVE_SCYTHE
        } else if rng.random_bool(0.5) {
            IDX_MOVE_TRI_ATTACK
        } else {
            IDX_MOVE_TRI_BURN
        }
    } else if roll < 65 {
        if !move_history.ends_with(&[IDX_MOVE_TRI_ATTACK as u8, IDX_MOVE_TRI_ATTACK as u8]) {
            IDX_MOVE_TRI_ATTACK
        } else if rng.random_bool(0.5) {
            IDX_MOVE_SCYTHE
        } else {
            IDX_MOVE_TRI_BURN
        }
    } else if last != Some(IDX_MOVE_TRI_BURN) {
        IDX_MOVE_TRI_BURN
    } else if rng.random_bool(0.5) {
        IDX_MOVE_SCYTHE
    } else {
        IDX_MOVE_TRI_ATTACK
    }
}
