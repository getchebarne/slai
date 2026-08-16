use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::CostScope;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_SNECKO_OIL: Entity = make_entity_potion(
    PotionName::SneckoOil,
    PotionRarity::Rare,
    true,
    &[
        Effect {
            kind: EffectKind::CardDraw { count: 5 },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::SetCostOverride {
                amount: 3,
                only_reduce: false,
                random: true,
                scope: CostScope::Combat,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Hand,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::All,
            },
        },
    ],
);
