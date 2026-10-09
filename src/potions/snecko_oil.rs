use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::CostScope;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static SNECKO_OIL: PotionTemplate = PotionTemplate {
    name: PotionName::SneckoOil,
    rarity: PotionRarity::Rare,
    combat_only: true,
    doubled: false,
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
                filters: &[],
                selection_kind: SelectionKind::All,
            },
        },
    ],
};
// Doubled
pub static SNECKO_OIL_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[
        Effect {
            kind: EffectKind::CardDraw { count: 10 },
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
                filters: &[],
                selection_kind: SelectionKind::All,
            },
        },
    ],
    ..SNECKO_OIL
};
