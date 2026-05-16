use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static BLOCK_POTION: Entity = make_entity_potion(
    PotionName::BlockPotion,
    PotionRarity::Common,
    false,
    true,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 12 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
);
