use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_LIQUID_MEMORIES: Entity = make_entity_potion(
    PotionName::LiquidMemories,
    PotionRarity::Uncommon,
    false,
    true,
    &[Effect {
        kind: EffectKind::LiquidMemories,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::PileDiscard,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    }],
);
