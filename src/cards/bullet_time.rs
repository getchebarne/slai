use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

// BulletTime: 3-cost (2+) Skill, no target. Sets every card in hand to cost
// 0 this turn (BulletTimeProc writes card_cost_override) and applies NoDraw
// (debuff that early-returns CardDraw). Both effects are scoped to the turn
// BulletTime is played — TurnEnd zeros card_cost_override across all cards
// and pushes ModifierRemove{NoDraw}.
pub static BULLET_TIME: Entity = make_entity_card(
    CardName::BulletTime,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    3,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BulletTimeProc,
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NoDraw,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
);
// Upgraded: cost 3 -> 2
pub static BULLET_TIME_PLUS: Entity = make_entity_card(
    CardName::BulletTime,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    2,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BulletTimeProc,
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::NoDraw,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
);
