use crate::cards::make_entity_card;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

const fn upgrade_all(pool: CandidatePool) -> Effect {
    Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: pool,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::All,
        },
    }
}

pub static APOTHEOSIS: Entity = make_entity_card(
    CardName::Apotheosis,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Rare,
    2,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    &[
        upgrade_all(CandidatePool::Hand),
        upgrade_all(CandidatePool::PileDraw),
        upgrade_all(CandidatePool::PileDiscard),
        upgrade_all(CandidatePool::PileExhaust),
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static APOTHEOSIS_PLUS: Entity = Entity {
    card_upgraded: true,
    card_cost: 1, // -1 cost
    ..APOTHEOSIS
};
