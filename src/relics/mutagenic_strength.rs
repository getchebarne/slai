use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Combat start: +3 Strength, lost at the end of the first turn
pub static MUTAGENIC_STRENGTH: RelicTemplate = RelicTemplate {
    name: RelicName::MutagenicStrength,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[
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
};
