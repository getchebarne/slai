use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::modifier::ModifierKind;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static BAG_OF_MARBLES: Entity = make_entity_relic(
    RelicName::BagOfMarbles,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Vulnerable,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
);
