use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_EXPLOSIVE: Entity = make_entity_potion(
    PotionName::ExplosivePotion,
    PotionRarity::Common,
    false,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 10 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters {
                filter: CandidatePoolMonstersFilter::All,
            },
            selection_kind: SelectionKind::All,
        },
    }],
);
