use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_BLESSING_OF_THE_FORGE: PotionTemplate = PotionTemplate {
    name: PotionName::BlessingOfTheForge,
    rarity: PotionRarity::Common,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Hand,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::All,
        },
    }],
};
