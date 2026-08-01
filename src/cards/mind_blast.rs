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

pub static MIND_BLAST: Entity = make_entity_card(
    CardName::MindBlast,
    CardKind::Attack,
    CardColor::Colorless,
    CardRarity::Uncommon,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    true,
    true,
    &[Effect {
        kind: EffectKind::DamageMindBlast,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters,
            filter: CandidateFilter::Picked,
            selection_kind: SelectionKind::Single,
        },
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static MIND_BLAST_PLUS: Entity = Entity {
    card_upgraded: true,
    card_cost: 1, // -1 cost
    ..MIND_BLAST
};
