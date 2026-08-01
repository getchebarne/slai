use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::modifier::ModifierKind;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_ESSENCE_OF_STEEL: Entity = make_entity_potion(
    PotionName::EssenceOfSteel,
    PotionRarity::Uncommon,
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::PlatedArmor,
            stacks: 4,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
);
