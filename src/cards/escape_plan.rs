use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::make_entity_card;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

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
                candidate_pool: CandidatePool::Character,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Single,
            },
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
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
