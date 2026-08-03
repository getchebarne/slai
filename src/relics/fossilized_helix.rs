use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::modifier::ModifierKind;
use crate::types::RelicName;
use crate::types::RelicTier;

// Start each combat with 1 Buffer (prevent the next HP loss)
// See:
//    - `process_effect_combat_start.rs`
pub static FOSSILIZED_HELIX: Entity = make_entity_relic(
    RelicName::FossilizedHelix,
    RelicTier::Rare,
    0,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Buffer,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
