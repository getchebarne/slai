use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static FIRE_POTION: Entity = make_entity_potion(
    PotionName::FirePotion,
    PotionRarity::Common,
    true,
    true,
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 20 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::MonsterPicked,
            selection_kind: SelectionKind::Single,
        },
    }],
);
