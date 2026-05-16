use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

// Matches StS `increaseMaxHp(5, true)` which raises the cap AND heals
pub static FRUIT_JUICE: Entity = make_entity_potion(
    PotionName::FruitJuice,
    PotionRarity::Rare,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::MaxHealthGain { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::HealthGain { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
    ],
);
