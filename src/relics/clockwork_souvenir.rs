use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static CLOCKWORK_SOUVENIR: RelicTemplate = RelicTemplate {
    name: RelicName::ClockworkSouvenir,
    tier: RelicTier::Shop,
    counter_init: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Artifact,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
