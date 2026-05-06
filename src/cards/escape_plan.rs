use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{CardCostKind, Entity, PlayRestriction, make_entity_card};
use crate::types::{CardColor, CardKind, CardName, CardRarity};

pub static ESCAPE_PLAN: Entity = make_entity_card(
    CardName::EscapePlan,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::EscapePlanCheck { block: 3 },
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
pub static ESCAPE_PLAN_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = ESCAPE_PLAN.card_effects;
        a[1].kind = EffectKind::EscapePlanCheck { block: 5 }; // +2 block
        a
    },
    ..ESCAPE_PLAN
};
