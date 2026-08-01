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
use crate::types::CardPile;
use crate::types::CardRarity;

pub static SECRET_TECHNIQUE: Entity = make_entity_card(
    CardName::SecretTechnique,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::CardMove {
            pile: CardPile::Hand,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::PileDraw,
            filter: CandidateFilter::KindSkill,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static SECRET_TECHNIQUE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_exhaust: false, // Doesn't exhaust
    ..SECRET_TECHNIQUE
};
