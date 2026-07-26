use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static BLESSING_OF_THE_FORGE: Entity = make_entity_potion(
    PotionName::BlessingOfTheForge,
    PotionRarity::Common,
    false,
    true,
    &[Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Hand {
                filter: CandidatePoolCardFilter::Any,
            },
            selection_kind: SelectionKind::All,
        },
    }],
);
