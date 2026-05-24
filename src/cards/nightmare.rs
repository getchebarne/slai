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

pub static NIGHTMARE: Entity = make_entity_card(
    CardName::Nightmare,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    3,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardNightmarePick,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Hand,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static NIGHTMARE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_cost: 2, // -1 cost
    ..NIGHTMARE
};
