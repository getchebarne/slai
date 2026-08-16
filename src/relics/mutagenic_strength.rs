use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// Combat start: +3 Strength, lost at the end of the first turn
pub static MUTAGENIC_STRENGTH: Entity = make_entity_relic(
    RelicName::MutagenicStrength,
    RelicTier::Special,
    0,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 3,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::LoseStrength,
                stacks: 3,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
);
