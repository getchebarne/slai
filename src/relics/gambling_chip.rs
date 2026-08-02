use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// At combat start, discard any number of Cards, then draw that many
// See:
//    - `process_effect_gamble.rs`
pub static GAMBLING_CHIP: Entity = make_entity_relic(
    RelicName::GamblingChip,
    RelicTier::Rare,
    0,
    &[Effect {
        kind: EffectKind::Gamble {
            choose_discards: true,
            discards_before: None,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
