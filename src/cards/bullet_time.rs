use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::modifier::ModifierKind;
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static BULLET_TIME: Entity = make_entity_card(
    CardName::BulletTime,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    3,
    CardCostKind::Fixed,
    false,
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
                selection: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
    &[],
);
// Upgraded
pub static BULLET_TIME_PLUS: Entity = Entity {
    card_upgraded: true,
    card_cost: 2, // -1 cost
    ..BULLET_TIME
};
