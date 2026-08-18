use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::CostScope;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_SNECKO_OIL: PotionTemplate = PotionTemplate {
    name: PotionName::SneckoOil,
    rarity: PotionRarity::Rare,
    combat_only: true,
    effects: &[
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
};
