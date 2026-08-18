use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static SNAKE_RING: RelicTemplate = RelicTemplate {
    name: RelicName::SnakeRing,
    tier: RelicTier::Starter,
    counter_init: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::CardDraw { count: 2 },
        id_source: None,
        target: Target::Direct(None),
    }],
};
