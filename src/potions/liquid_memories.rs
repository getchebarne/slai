use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_LIQUID_MEMORIES: Entity = make_entity_potion(
    PotionName::LiquidMemories,
    PotionRarity::Uncommon,
    false,
    true,
    &[Effect {
        kind: EffectKind::CardMove {
            pile: CardPile::Hand,
            cost_zero: Some(CostScope::Turn),
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::PileDiscard,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    }],
);
